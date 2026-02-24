import { defineAction, ActionError } from "astro:actions";
import { z } from "astro:schema";

export const server = {
  shortenUrl: defineAction({
    input: z.object({
      url: z.preprocess(
        (val) => {
          if (typeof val !== "string") return val;
          let url = val.trim();

          if (!/^https?:\/\//i.test(url)) {
            url = `https://${url}`;
          }
          return url;
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
    handler: async (input) => {
      const apiUrl = import.meta.env.API_URL;

      if (!apiUrl) {
        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Server configuration incomplete (API_URL missing)",
        });
      }

      try {
        const response = await fetch(`${apiUrl}/`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ original_url: input.url }),
        });

        if (!response.ok) {
          const errorData = await response.json().catch(() => ({}));
          throw new ActionError({
            code:
              response.status === 400 ? "BAD_REQUEST" : "INTERNAL_SERVER_ERROR",
            message: errorData.message || "Backend failed to process the URL",
          });
        }

        const data = await response.json();

        return {
          short_code: data.short_code,
          original_url: data.original_url,
          short_url: data.short_url,
          created_at: data.created_at,
          expires_at: data.expires_at,
        };
      } catch (e) {
        if (e instanceof ActionError) throw e;

        throw new ActionError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Failed to connect to the shortening service",
        });
      }
    },
  }),
};
