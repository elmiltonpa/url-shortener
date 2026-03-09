import { ref, readonly } from "vue";
import type { UrlData } from "../types/url";

const STORAGE_KEY = "url_shortener_history";
const MAX_ITEMS = 10;

function isUrlData(item: unknown): item is UrlData {
  if (typeof item !== "object" || item === null) return false;
  const obj = item as Record<string, unknown>;
  return (
    typeof obj.short_code === "string" &&
    typeof obj.original_url === "string" &&
    typeof obj.short_url === "string" &&
    typeof obj.created_at === "string" &&
    typeof obj.expires_at === "string"
  );
}

function loadHistory(): UrlData[] {
  if (typeof window === "undefined") return [];
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (!saved) return [];

    const parsed: unknown = JSON.parse(saved);
    if (!Array.isArray(parsed)) return [];

    return parsed.filter(isUrlData);
  } catch {
    return [];
  }
}

const history = ref<UrlData[]>(loadHistory());

function saveHistory() {
  if (typeof window === "undefined") return;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(history.value));
}

export function useUrlHistory() {
  const addUrl = (url: UrlData) => {
    const filtered = history.value.filter(
      (i) => i.short_code !== url.short_code,
    );

    history.value = [url, ...filtered].slice(0, MAX_ITEMS);

    saveHistory();
  };

  const clearHistory = () => {
    history.value = [];
    if (typeof window !== "undefined") {
      localStorage.removeItem(STORAGE_KEY);
    }
  };

  return {
    history: readonly(history),
    addUrl,
    clearHistory,
  };
}
