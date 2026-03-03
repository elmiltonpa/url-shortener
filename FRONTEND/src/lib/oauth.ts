import type { AstroCookies } from "astro";

const OAUTH_STATE_COOKIE = "oauth_state";
const STATE_BYTE_LENGTH = 32;

export function generateOAuthState(cookies: AstroCookies): string {
  const bytes = crypto.getRandomValues(new Uint8Array(STATE_BYTE_LENGTH));
  const state = Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join(
    "",
  );

  cookies.set(OAUTH_STATE_COOKIE, state, {
    path: "/",
    httpOnly: true,
    secure: import.meta.env.PROD,
    sameSite: "lax",
    maxAge: 60 * 10,
  });

  return state;
}

export function validateOAuthState(
  cookies: AstroCookies,
  stateFromUrl: string | null,
): boolean {
  const storedState = cookies.get(OAUTH_STATE_COOKIE)?.value;

  cookies.delete(OAUTH_STATE_COOKIE, { path: "/" });

  if (!storedState || !stateFromUrl) {
    return false;
  }
  if (storedState.length !== stateFromUrl.length) {
    return false;
  }

  let mismatch = 0;
  for (let i = 0; i < storedState.length; i++) {
    mismatch |= storedState.charCodeAt(i) ^ stateFromUrl.charCodeAt(i);
  }

  return mismatch === 0;
}

export function buildGoogleAuthUrl(
  clientId: string,
  redirectUri: string,
  state: string,
): string {
  const params = new URLSearchParams({
    client_id: clientId,
    redirect_uri: redirectUri,
    response_type: "code",
    scope: "openid email profile",
    state,
    prompt: "select_account",
  });

  return `https://accounts.google.com/o/oauth2/v2/auth?${params.toString()}`;
}
