import { defineMiddleware } from "astro:middleware";

export const onRequest = defineMiddleware(async (context, next) => {
  const token = context.cookies.get("auth_token")?.value;
  const userDataCookie = context.cookies.get("user_data")?.value;

  context.locals.user = null;
  context.locals.token = token || null;

  if (token && userDataCookie) {
    try {
      const parsed = JSON.parse(userDataCookie);

      if (
        typeof parsed === "object" &&
        parsed !== null &&
        typeof parsed.id === "string" &&
        typeof parsed.email === "string" &&
        typeof parsed.username === "string"
      ) {
        context.locals.user = {
          id: parsed.id,
          email: parsed.email,
          username: parsed.username,
        };
      } else {
        context.cookies.delete("user_data", { path: "/" });
        context.cookies.delete("auth_token", { path: "/" });
      }
    } catch {
      context.cookies.delete("user_data", { path: "/" });
      context.cookies.delete("auth_token", { path: "/" });
    }
  } else if (token && !userDataCookie) {
    context.cookies.delete("auth_token", { path: "/" });
    context.locals.token = null;
  } else if (!token && userDataCookie) {
    context.cookies.delete("user_data", { path: "/" });
  }

  const { pathname } = context.url;
  const isAuthPage =
    pathname.startsWith("/login") || pathname.startsWith("/register");
  const isProtectedRoute =
    pathname.startsWith("/stats") || pathname.startsWith("/dashboard");

  if (isAuthPage && context.locals.user) {
    return context.redirect("/");
  }

  if (isProtectedRoute && !context.locals.user) {
    return context.redirect("/login");
  }

  return next();
});
