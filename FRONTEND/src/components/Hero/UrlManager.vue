<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { actions } from "astro:actions";
import { Loader2 } from "lucide-vue-next";
import UrlForm from "./UrlForm.vue";
import UrlList from "./UrlList.vue";
import { useUrlHistory } from "../../composables/useUrlHistory.ts";
import type { UrlData } from "../../types/url";

const props = defineProps<{
    isLoggedIn: boolean;
    initialServerLinks?: UrlData[];
}>();

const { history: localHistory, clearHistory } = useUrlHistory();

const serverLinks = ref<UrlData[]>(props.initialServerLinks || []);
const isSyncing = ref(false);

const displayLinks = computed((): readonly UrlData[] => {
    return props.isLoggedIn ? serverLinks.value : localHistory.value;
});

const fetchServerLinks = async () => {
    const { data, error } = await actions.getUserUrls({ page: 1, per_page: 10 });
    if (!error && data?.urls) {
        serverLinks.value = data.urls;
    }
};

const syncAnonymousLinks = async () => {
    isSyncing.value = true;
    try {
        const codes = localHistory.value.map((url) => url.short_code);
        const { data, error } = await actions.claimAnonymousUrls({ codes });

        if (!error && data?.success) {
            clearHistory();
            await fetchServerLinks();
        }
    } catch (e) {
        console.warn("[UrlManager] Failed to sync anonymous links:", e);
    } finally {
        isSyncing.value = false;
    }
};

onMounted(() => {
    if (props.isLoggedIn && localHistory.value.length > 0) {
        syncAnonymousLinks();
    }
});

const handleLinkCreated = (newLink: UrlData | null) => {
    if (newLink && props.isLoggedIn) {
        serverLinks.value.unshift(newLink);
    }
};
</script>

<template>
    <div class="w-full flex flex-col items-center">
        <UrlForm :isLoggedIn="isLoggedIn" @linkCreated="handleLinkCreated" />
        
        <div v-if="isSyncing && serverLinks.length === 0" class="py-12 flex flex-col items-center gap-3 text-muted-foreground animate-in fade-in duration-500">
            <Loader2 class="h-8 w-8 animate-spin text-primary" />
            <p class="text-sm font-medium text-rust">Sincronizando tus links...</p>
        </div>

        <UrlList 
            v-else-if="displayLinks.length > 0" 
            :links="displayLinks" 
            :class="{ 'opacity-50 pointer-events-none transition-opacity': isSyncing }"
        />
    </div>
</template>
