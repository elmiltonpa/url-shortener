<script setup lang="ts">
import UrlCard from "./UrlCard.vue";
import type { UrlData } from "../../types/url";

defineProps<{
    links: readonly UrlData[];
}>();
</script>

<template>
    <div class="mt-8 w-full max-w-xl space-y-4">
        <TransitionGroup
            enter-active-class="transition duration-300 ease-out"
            enter-from-class="opacity-0 -translate-y-3 scale-95"
            enter-to-class="opacity-100 translate-y-0 scale-100"
            leave-active-class="transition duration-200 ease-in"
            leave-from-class="opacity-100 translate-y-0 scale-100"
            leave-to-class="opacity-0 translate-y-3 scale-95"
            move-class="transition duration-300 ease-in-out"
        >
            <UrlCard 
                v-for="link in links" 
                :key="link.short_code"
                :shortenedLink="link.short_url"
                :originalUrl="link.original_url"
            />
        </TransitionGroup>
    </div>
</template>
