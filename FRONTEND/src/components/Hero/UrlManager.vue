<script setup lang="ts">
import { ref, computed } from "vue";
import UrlForm from "./UrlForm.vue";
import UrlList from "./UrlList.vue";
import { useUrlHistory } from "../../composables/useUrlHistory.ts";
import type { UrlData } from "../../types/url";

const props = defineProps<{
    isLoggedIn: boolean;
    initialServerLinks?: UrlData[];
}>();

const { history: localHistory } = useUrlHistory();

const serverLinks = ref<UrlData[]>(props.initialServerLinks || []);

const displayLinks = computed((): readonly UrlData[] => {
    return props.isLoggedIn ? serverLinks.value : localHistory.value;
});

const handleLinkCreated = (newLink: UrlData | null) => {
    if (newLink && props.isLoggedIn) {
        serverLinks.value.unshift(newLink);
    }
};
</script>

<template>
    <div class="w-full flex flex-col items-center">
        <UrlForm 
            :isLoggedIn="isLoggedIn" 
            @linkCreated="handleLinkCreated" 
        />
        <UrlList 
            v-if="displayLinks.length > 0"
            :links="displayLinks" 
        />
    </div>
</template>
