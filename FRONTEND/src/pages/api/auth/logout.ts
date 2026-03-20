import type { APIRoute } from "astro";

export const POST: APIRoute = (context) => {
  // Clear cookies by setting them with empty value and immediate expiration
  const cookieOptions = {
    path: "/",
    httpOnly: true,
    secure: import.meta.env.PROD,
    sameSite: "lax" as const,
    maxAge: 0,
    expires: new Date(0),
  };

  context.cookies.set("auth_token", "", cookieOptions);
  context.cookies.set("user_data", "", cookieOptions);

  // Return JSON response for API calls
  return new Response(JSON.stringify({ success: true }), {
    status: 200,
    headers: {
      "Content-Type": "application/json",
    },
  });
};

// Also support GET for simple redirects
export const GET: APIRoute = (context) => {
  const cookieOptions = {
    path: "/",
    httpOnly: true,
    secure: import.meta.env.PROD,
    sameSite: "lax" as const,
    maxAge: 0,
    expires: new Date(0),
  };

  context.cookies.set("auth_token", "", cookieOptions);
  context.cookies.set("user_data", "", cookieOptions);

  // Redirect to home page
  return context.redirect("/", 302);
};
