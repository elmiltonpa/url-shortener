<script setup lang="ts">
import { Link2, MousePointerClick, TrendingUp } from "lucide-vue-next";
import type { DashboardStats } from "../../types/url";

defineProps<{
    stats: DashboardStats;
    loading: boolean;
}>();
</script>

<template>
    <div
        class="grid grid-cols-1 md:grid-cols-3 gap-6 w-full max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 mb-12"
    >
        <div
            class="relative overflow-hidden group rounded-2xl border border-border bg-card/50 p-6 backdrop-blur-sm transition-all hover:border-rust/30"
        >
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm font-medium text-muted-foreground mb-1">
                        Total Links
                    </p>
                    <h3 class="text-3xl font-bold tracking-tight">
                        <template v-if="loading">
                            <div
                                class="h-8 w-16 bg-muted animate-pulse rounded"
                            ></div>
                        </template>
                        <template v-else>
                            {{ stats.total_links }}
                        </template>
                    </h3>
                </div>
                <div class="rounded-xl bg-rust/10 p-3 text-rust">
                    <Link2 class="h-6 w-6" />
                </div>
            </div>
            <div class="mt-4 flex items-center text-xs text-muted-foreground">
                <span class="flex items-center gap-1 text-rust">
                    <TrendingUp class="h-3 w-3" />
                </span>
                <span>All time</span>
            </div>
        </div>

        <div
            class="relative overflow-hidden group rounded-2xl border border-border bg-card/50 p-6 backdrop-blur-sm transition-all hover:border-rust/30"
        >
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm font-medium text-muted-foreground mb-1">
                        Total Clicks
                    </p>
                    <h3 class="text-3xl font-bold tracking-tight">
                        <template v-if="loading">
                            <div
                                class="h-8 w-24 bg-muted animate-pulse rounded"
                            ></div>
                        </template>
                        <template v-else>
                            {{ stats.total_clicks }}
                        </template>
                    </h3>
                </div>
                <div class="rounded-xl bg-rust/10 p-3 text-rust">
                    <MousePointerClick class="h-6 w-6" />
                </div>
            </div>
            <div class="mt-4 flex items-center text-xs text-muted-foreground">
                <span class="flex items-center gap-1 text-emerald-500">
                    <TrendingUp class="h-3 w-3" />
                    Real-time
                </span>
                <span class="ml-1.5">engagement tracker</span>
            </div>
        </div>

        <div
            class="relative overflow-hidden group rounded-2xl border border-border bg-card/50 p-6 backdrop-blur-sm transition-all hover:border-rust/30"
        >
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm font-medium text-muted-foreground mb-1">
                        Top Performing
                    </p>
                    <h3
                        class="text-xl font-bold tracking-tight truncate max-w-45"
                    >
                        <template v-if="loading">
                            <div
                                class="h-7 w-32 bg-muted animate-pulse rounded"
                            ></div>
                        </template>
                        <template v-else-if="stats.top_link">
                            /{{ stats.top_link.short_code }}
                        </template>
                        <template v-else> N/A </template>
                    </h3>
                </div>
                <div class="rounded-xl bg-rust/10 p-3 text-rust">
                    <TrendingUp class="h-6 w-6" />
                </div>
            </div>
            <div class="mt-4 flex items-center text-xs text-muted-foreground">
                <template v-if="stats.top_link">
                    <span class="text-rust font-bold">{{
                        stats.top_link.click_count
                    }}</span>
                    <span class="ml-1.5">clicks on this link</span>
                </template>
                <template v-else>
                    <span class="ml-1.5">No link data available</span>
                </template>
            </div>
        </div>
    </div>
</template>
