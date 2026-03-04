import type { APIRoute } from "astro";
import { generateOAuthState, buildGoogleAuthUrl } from "../../../lib/oauth";

export const GET: APIRoute = (context) => {
  const clientId = import.meta.env.PUBLIC_GOOGLE_CLIENT_ID;

  if (!clientId) {
    return context.redirect("/error?code=500", 302);
  }

  const redirectUri = `${context.url.origin}/auth/google/callback`;
  const state = generateOAuthState(context.cookies);
  const googleUrl = buildGoogleAuthUrl(clientId, redirectUri, state);

  return context.redirect(googleUrl, 302);
};
