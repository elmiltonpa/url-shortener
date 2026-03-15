<script setup lang="ts">
import { BarChart3, RefreshCw, X } from "lucide-vue-next";
import type { UrlStatsResponse } from "../../../types/url";

defineProps<{
    shortCode: string;
    stats: UrlStatsResponse | null;
    loading: boolean;
}>();

const emit = defineEmits<{
    (e: "refresh"): void;
    (e: "close"): void;
}>();
</script>

<template>
    <div
        class="flex items-center justify-between gap-4 px-6 py-5 border-b border-border shrink-0"
    >
        <div class="flex items-center gap-3 min-w-0">
            <div class="rounded-xl bg-rust/10 p-2.5 text-rust shrink-0">
                <BarChart3 class="h-5 w-5" />
            </div>
            <div class="min-w-0">
                <h2
                    id="stats-modal-title"
                    class="text-lg font-bold tracking-tight"
                >
                    Link Statistics
                </h2>
                <code class="text-xs text-rust font-mono"
                    >/{{ shortCode }}</code
                >
            </div>
        </div>
        <div class="flex items-center gap-2 shrink-0">
            <button
                v-if="stats && !loading"
                @click="emit('refresh')"
                class="rounded-xl p-2 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
                title="Refresh stats"
                aria-label="Refresh stats"
            >
                <RefreshCw class="h-4 w-4" />
            </button>
            <button
                @click="emit('close')"
                class="rounded-xl p-2 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
                aria-label="Close stats panel"
            >
                <X class="h-5 w-5" />
            </button>
        </div>
    </div>
</template>
