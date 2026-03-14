export function formatDate(dateStr: string | null | undefined): string {
  if (!dateStr) return "N/A";
  return new Date(dateStr).toLocaleDateString(undefined, {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

export function formatDateTime(dateStr: string): string {
  return new Date(dateStr).toLocaleString(undefined, {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export interface ExpiryBadge {
  label: string;
  cls: string;
}

export function getExpiryBadge(
  expiresAt: string | null | undefined,
): ExpiryBadge | null {
  if (!expiresAt) return null;

  const diffMs = new Date(expiresAt).getTime() - Date.now();
  const days = Math.ceil(diffMs / 86_400_000);

  if (days <= 0) {
    return {
      label: "Expired",
      cls: "bg-red-500/15 text-red-400 border-red-500/20",
    };
  }
  if (days <= 3) {
    return {
      label: `${days}d left`,
      cls: "bg-orange-500/15 text-orange-400 border-orange-500/20",
    };
  }
  if (days <= 7) {
    return {
      label: `${days}d left`,
      cls: "bg-yellow-500/15 text-yellow-400 border-yellow-500/20",
    };
  }
  return {
    label: `${days}d left`,
    cls: "bg-muted text-muted-foreground border-border",
  };
}
