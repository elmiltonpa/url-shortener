import { ref, onMounted, computed } from "vue";
import { actions } from "astro:actions";
import type { UrlData, DashboardStats } from "../types/url";

export function useDashboard() {
    const urls = ref<UrlData[]>([]);
    const loading = ref(true);
    const searchQuery = ref("");
    const errorMessage = ref<string | null>(null);

    const filteredUrls = computed(() => {
        if (!searchQuery.value) return urls.value;
        const query = searchQuery.value.toLowerCase();
        return urls.value.filter(u => 
            u.original_url.toLowerCase().includes(query) || 
            u.short_code.toLowerCase().includes(query)
        );
    });

    const stats = computed<DashboardStats>(() => {
        const total_links = urls.value.length;
        const total_clicks = urls.value.reduce((acc, url) => acc + url.click_count, 0);
        const top_link = [...urls.value].sort((a, b) => b.click_count - a.click_count)[0] || null;
        
        return {
            total_links,
            total_clicks,
            top_link
        };
    });

    const fetchUrls = async () => {
        loading.value = true;
        errorMessage.value = null;
        try {
            const { data, error } = await actions.getUserUrls({ page: 1, per_page: 50 });
            if (data && data.urls) {
                const frontendOrigin = window.location.origin;
                urls.value = data.urls.map((u: any) => ({
                    ...u,
                    short_url: `${frontendOrigin}/${u.short_code}`
                }));
            } else if (error) {
                errorMessage.value = error.message || "Failed to load links. Please try again later.";
            }
        } catch (e) {
            errorMessage.value = "A connection error occurred. Check your internet.";
        } finally {
            loading.value = false;
        }
    };

    onMounted(() => {
        fetchUrls();
    });

    return {
        urls,
        loading,
        searchQuery,
        filteredUrls,
        stats,
        fetchUrls,
        errorMessage
    };
}
