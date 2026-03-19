import { defineAction, ActionError } from "astro:actions";
import type { AstroCookies } from "astro";
import { z } from "astro:schema";

const STATUS_CODE_MAP: Record<number, ActionError["code"]> = {
  400: "BAD_REQUEST",
  401: "UNAUTHORIZED",
  403: "FORBIDDEN",
  404: "NOT_FOUND",
  409: "CONFLICT",
  429: "TOO_MANY_REQUESTS",
};

function mapStatusToCode(status: number): ActionError["code"] {
  return STATUS_CODE_MAP[status] ?? "INTERNAL_SERVER_ERROR";
}

async function handleBackendResponse(response: Response, defaultError: string) {
  if (!response.ok) {
    let errorData: Record<string, string> = {};
    try {
      errorData = await response.json();
    } catch {
      console.error(
        `[backend] Non-JSON error response: ${response.status} ${response.statusText}`,
      );
    }
    throw new ActionError({
      code: mapStatusToCode(response.status),
      message: errorData.error || defaultError,
    });
  }
  return response.json();
}

function getApiUrl(): string {
  const apiUrl = import.meta.env.API_URL;
  if (!apiUrl) {
    throw new ActionError({
      code: "INTERNAL_SERVER_ERROR",
      message: "Server configuration error",
    });
  }
  return apiUrl;
}

const AUTH_COOKIE_OPTIONS = {
  path: "/",
  httpOnly: true,
  secure: import.meta.env.PROD,
  sameSite: "lax",
  maxAge: 60 * 60 * 24,
} as const;

interface AuthResponse {
  token: string;
  user: { id: string; email: string; username: string };
}

function setAuthCookies(cookies: AstroCookies, data: AuthResponse): void {
  cookies.set("auth_token", data.token, AUTH_COOKIE_OPTIONS);

  cookies.set("user_data", JSON.stringify(data.user), {
    ...AUTH_COOKIE_OPTIONS,
  });
}

function clearAuthCookies(cookies: AstroCookies): void {
  const options = {
    path: "/",
    httpOnly: true,
    secure: import.meta.env.PROD,
    sameSite: "lax" as const,
  };
  cookies.delete("auth_token", options);
  cookies.delete("user_data", options);
}

export const server = {
  claimAnonymousUrls: defineAction({
    input: z.object({
      codes: z.array(z.string()),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();
      const token = context.cookies.get("auth_token")?.value;

      if (!token)
        throw new ActionError({
          code: "UNAUTHORIZED",
          message: "Not authenticated",
        });

      try {
        const response = await fetch(`${apiUrl}/user/claim`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify(input),
        });

        return await handleBackendResponse(response, "Failed to claim URLs");
      } catch (e) {
        if (e instanceof ActionError) throw e;
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Connection error",
        });
      }
    },
  }),

  getUserUrls: defineAction({
    input: z.object({
      page: z.number().optional().default(1),
      per_page: z.number().optional().default(10),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();

      try {
        const token = context.cookies.get("auth_token")?.value;
        const headers: Record<string, string> = {
          "Content-Type": "application/json",
        };
        if (token) headers["Authorization"] = `Bearer ${token}`;

        const response = await fetch(
          `${apiUrl}/user/urls?page=${input.page}&per_page=${input.per_page}`,
          {
            method: "GET",
            headers,
          },
        );

        const data = await handleBackendResponse(
          response,
          "Failed to fetch user URLs",
        );

        return data;
      } catch (e) {
        if (e instanceof ActionError) throw e;
        console.error("[getUserUrls] Failed to reach backend:", e);
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Failed to connect to the shortening service",
        });
      }
    },
  }),

  shortenUrl: defineAction({
    input: z.object({
      url: z.preprocess(
        (val) => {
          if (typeof val !== "string") return val;
          const trimmed = val.trim();
          return /^https?:\/\//i.test(trimmed) ? trimmed : `https://${trimmed}`;
        },
        z
          .string()
          .url("Invalid URL format")
          .refine(
            (val) => {
              try {
                const parsed = new URL(val);
                const hostParts = parsed.hostname.split(".");
                return (
                  hostParts.length >= 2 &&
                  hostParts[hostParts.length - 1].length >= 2
                );
              } catch {
                return false;
              }
            },
            {
              message: "URL must have a valid domain and TLD (e.g., .com, .io)",
            },
          ),
      ),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();

      try {
        const token = context.cookies.get("auth_token")?.value;
        const headers: Record<string, string> = {
          "Content-Type": "application/json",
        };
        if (token) headers["Authorization"] = `Bearer ${token}`;

        const response = await fetch(`${apiUrl}/`, {
          method: "POST",
          headers,
          body: JSON.stringify({ original_url: input.url }),
        });

        const data = await handleBackendResponse(
          response,
          "Backend failed to process the URL",
        );

        const frontendOrigin =
          import.meta.env.PUBLIC_APP_DOMAIN ||
          new URL(context.request.url).origin;

        return {
          short_code: data.short_code,
          original_url: data.original_url,
          short_url: `${frontendOrigin}/${data.short_code}`,
          click_count: data.click_count,
          created_at: data.created_at,
          expires_at: data.expires_at,
        };
      } catch (e) {
        if (e instanceof ActionError) throw e;
        console.error("[shortenUrl] Failed to reach backend:", e);
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Failed to connect to the shortening service",
        });
      }
    },
  }),

  register: defineAction({
    input: z.object({
      username: z
        .string()
        .min(3, "Username must be at least 3 characters")
        .max(50, "Username must be at most 50 characters"),
      email: z.string().email("Invalid email format"),
      password: z.string().min(6, "Password must be at least 6 characters"),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();

      try {
        const response = await fetch(`${apiUrl}/auth/register`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(input),
        });

        const data = await handleBackendResponse(
          response,
          "Registration failed",
        );

        setAuthCookies(context.cookies, data);
        return { user: data.user };
      } catch (e) {
        if (e instanceof ActionError) throw e;
        console.error("[register] Failed to reach backend:", e);
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "An unexpected error occurred during registration",
        });
      }
    },
  }),

  login: defineAction({
    input: z.object({
      email: z.string().email("Invalid email format"),
      password: z.string().min(1, "Password is required"),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();

      try {
        const response = await fetch(`${apiUrl}/auth/login`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(input),
        });

        const data = await handleBackendResponse(response, "Login failed");

        setAuthCookies(context.cookies, data);
        return { user: data.user };
      } catch (e) {
        if (e instanceof ActionError) throw e;
        console.error("[login] Failed to reach backend:", e);
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "An unexpected error occurred during login",
        });
      }
    },
  }),

  loginWithGoogle: defineAction({
    input: z.object({
      code: z.string().min(1, "Authorization code is required"),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();

      try {
        const response = await fetch(`${apiUrl}/auth/google`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ code: input.code }),
        });

        const data = await handleBackendResponse(
          response,
          "Google login failed",
        );

        setAuthCookies(context.cookies, data);
        return { user: data.user };
      } catch (e) {
        if (e instanceof ActionError) throw e;
        console.error("[loginWithGoogle] Failed to reach backend:", e);
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "An unexpected error occurred during Google login",
        });
      }
    },
  }),

  logout: defineAction({
    handler: async (_, context) => {
      clearAuthCookies(context.cookies);
      return { success: true };
    },
  }),

  getUrlStats: defineAction({
    input: z.object({
      code: z.string().min(1),
      page: z.number().optional().default(1),
      per_page: z.number().optional().default(20),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();
      const token = context.cookies.get("auth_token")?.value;

      if (!token)
        throw new ActionError({
          code: "UNAUTHORIZED",
          message: "Not authenticated",
        });

      try {
        const response = await fetch(
          `${apiUrl}/stats/${input.code}?page=${input.page}&per_page=${input.per_page}`,
          {
            method: "GET",
            headers: {
              Authorization: `Bearer ${token}`,
            },
          },
        );

        return await handleBackendResponse(
          response,
          "Failed to fetch URL stats",
        );
      } catch (e) {
        if (e instanceof ActionError) throw e;
        console.error("[getUrlStats] Failed to reach backend:", e);
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Failed to connect to the service",
        });
      }
    },
  }),

  deleteUrl: defineAction({
    input: z.object({
      code: z.string().min(1),
    }),
    handler: async (input, context) => {
      const apiUrl = getApiUrl();
      const token = context.cookies.get("auth_token")?.value;

      if (!token)
        throw new ActionError({
          code: "UNAUTHORIZED",
          message: "Not authenticated",
        });

      try {
        const response = await fetch(`${apiUrl}/user/urls/${input.code}`, {
          method: "DELETE",
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (response.status === 204) return { success: true };

        return await handleBackendResponse(response, "Failed to delete URL");
      } catch (e) {
        if (e instanceof ActionError) throw e;
        console.error("[deleteUrl] Failed to reach backend:", e);
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Failed to connect to the service",
        });
      }
    },
  }),
};
