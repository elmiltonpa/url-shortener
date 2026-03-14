import { ref, computed, watch, onMounted } from "vue";
import { actions } from "astro:actions";
import type { UrlData, DashboardStats } from "../types/url";

type ApiUrlData = Omit<UrlData, "short_url">;

const PER_PAGE = 10;

export function useDashboard() {
  const urls = ref<UrlData[]>([]);
  const loading = ref(true);
  const searchQuery = ref("");
  const errorMessage = ref<string | null>(null);
  const currentPage = ref(1);
  const perPage = ref(PER_PAGE);
  const deletingCodes = ref<Set<string>>(new Set());

  watch(searchQuery, () => {
    currentPage.value = 1;
  });

  const filteredUrls = computed(() => {
    if (!searchQuery.value.trim()) return urls.value;
    const query = searchQuery.value.toLowerCase().trim();
    return urls.value.filter(
      (u) =>
        u.original_url.toLowerCase().includes(query) ||
        u.short_code.toLowerCase().includes(query),
    );
  });

  const totalPages = computed(() =>
    Math.max(1, Math.ceil(filteredUrls.value.length / perPage.value)),
  );

  const paginatedUrls = computed(() => {
    const start = (currentPage.value - 1) * perPage.value;
    return filteredUrls.value.slice(start, start + perPage.value);
  });

  const stats = computed<DashboardStats>(() => {
    const total_links = urls.value.length;
    const total_clicks = urls.value.reduce(
      (acc, url) => acc + url.click_count,
      0,
    );
    const top_link =
      [...urls.value].sort((a, b) => b.click_count - a.click_count)[0] || null;

    return { total_links, total_clicks, top_link };
  });

  const fetchUrls = async () => {
    loading.value = true;
    errorMessage.value = null;
    try {
      const { data, error } = await actions.getUserUrls({
        page: 1,
        per_page: 100,
      });
      if (data && data.urls) {
        const frontendOrigin = window.location.origin;
        urls.value = data.urls.map((u: ApiUrlData) => ({
          ...u,
          short_url: `${frontendOrigin}/${u.short_code}`,
        }));
      } else if (error) {
        errorMessage.value =
          error.message || "Failed to load links. Please try again later.";
      }
    } catch {
      errorMessage.value = "A connection error occurred. Check your internet.";
    } finally {
      loading.value = false;
    }
  };

  const deleteUrl = async (shortCode: string): Promise<boolean> => {
    deletingCodes.value = new Set([...deletingCodes.value, shortCode]);
    try {
      const { data, error } = await actions.deleteUrl({ code: shortCode });
      if (error) return false;
      if (data?.success) {
        urls.value = urls.value.filter((u) => u.short_code !== shortCode);
        if (currentPage.value > totalPages.value && currentPage.value > 1) {
          currentPage.value = totalPages.value;
        }
        return true;
      }
      return false;
    } catch {
      return false;
    } finally {
      const next = new Set(deletingCodes.value);
      next.delete(shortCode);
      deletingCodes.value = next;
    }
  };

  const setPage = (page: number) => {
    if (page < 1 || page > totalPages.value) return;
    currentPage.value = page;
  };

  onMounted(() => {
    fetchUrls();
  });

  return {
    urls,
    loading,
    searchQuery,
    filteredUrls,
    paginatedUrls,
    stats,
    currentPage,
    totalPages,
    perPage,
    deletingCodes,
    fetchUrls,
    deleteUrl,
    setPage,
    errorMessage,
  };
}
