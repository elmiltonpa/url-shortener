export function parseBrowser(ua: string | null): string {
  if (!ua) return "Unknown";
  if (/Edg\//.test(ua)) return "Edge";
  if (/OPR\/|Opera/.test(ua)) return "Opera";
  if (/Firefox\//.test(ua)) return "Firefox";
  if (/Chrome\//.test(ua)) return "Chrome";
  if (/Safari\//.test(ua)) return "Safari";
  return "Other";
}

export function parseDevice(ua: string | null): "Mobile" | "Desktop" {
  if (!ua) return "Desktop";
  return /Mobile|Android|iPhone|iPad|iPod/.test(ua) ? "Mobile" : "Desktop";
}

export function cleanReferrer(referrer: string | null | undefined): string {
  if (!referrer) return "Direct";
  try {
    return new URL(referrer).hostname.replace(/^www\./, "");
  } catch {
    return referrer.length > 40 ? referrer.slice(0, 40) + "…" : referrer;
  }
}
