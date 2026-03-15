<script setup lang="ts">
import { ref, computed } from "vue";
import {
    Clock,
    Monitor,
    Smartphone,
    ChevronLeft,
    ChevronRight,
    MousePointerClick,
} from "lucide-vue-next";
import type { StatEntry } from "../../../types/url";
import {
    formatDateTime,
} from "../../../utils/formatters";
import {
    parseBrowser,
    parseDevice,
    cleanReferrer,
} from "../../../utils/analytics";

const props = defineProps<{
    clicks: StatEntry[];
}>();

const PER_PAGE = 10;
const currentPage = ref(1);

const totalPages = computed(() =>
    Math.max(1, Math.ceil(props.clicks.length / PER_PAGE)),
);

const paginatedClicks = computed(() => {
    if (!props.clicks.length) return [];
    const start = (currentPage.value - 1) * PER_PAGE;
    return props.clicks.slice(start, start + PER_PAGE);
});

function prevPage() {
    if (currentPage.value > 1) currentPage.value--;
}
function nextPage() {
    if (currentPage.value < totalPages.value) currentPage.value++;
}
</script>

<template>
    <div
        v-if="clicks.length"
        class="rounded-2xl border border-border bg-muted/20 overflow-hidden"
    >
        <div
            class="flex items-center justify-between px-4 py-3 border-b border-border"
        >
            <p
                class="text-xs font-semibold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5"
            >
                <Clock class="h-3.5 w-3.5" /> Recent Clicks
            </p>
            <span class="text-xs text-muted-foreground">
                {{ clicks.length }} total
            </span>
        </div>

        <div class="divide-y divide-border" role="list" aria-label="Click history">
            <div
                v-for="click in paginatedClicks"
                :key="click.id"
                class="flex items-center gap-3 px-4 py-2.5 hover:bg-muted/30 transition-colors"
                role="listitem"
            >
                <component
                    :is="
                        parseDevice(click.user_agent) === 'Mobile'
                            ? Smartphone
                            : Monitor
                    "
                    class="h-3.5 w-3.5 text-muted-foreground shrink-0"
                    aria-hidden="true"
                />

                <div class="flex-1 min-w-0">
                    <p class="text-sm text-foreground truncate">
                        {{ cleanReferrer(click.referrer) }}
                    </p>
                    <p class="text-xs text-muted-foreground">
                        {{ parseBrowser(click.user_agent) }}
                        <template v-if="click.country_code">
                            ·
                            {{ click.country_code }}</template
                        >
                    </p>
                </div>

                <time
                    class="text-xs text-muted-foreground shrink-0 tabular-nums"
                    :datetime="click.created_at"
                >
                    {{ formatDateTime(click.created_at) }}
                </time>
            </div>
        </div>

        <div
            v-if="totalPages > 1"
            class="flex items-center justify-between px-4 py-3 border-t border-border"
        >
            <button
                @click="prevPage"
                :disabled="currentPage === 1"
                class="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer"
                aria-label="Previous page"
            >
                <ChevronLeft class="h-3.5 w-3.5" />
                Prev
            </button>
            <span class="text-xs text-muted-foreground" aria-live="polite">
                Page {{ currentPage }} of
                {{ totalPages }}
            </span>
            <button
                @click="nextPage"
                :disabled="currentPage === totalPages"
                class="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer"
                aria-label="Next page"
            >
                Next
                <ChevronRight class="h-3.5 w-3.5" />
            </button>
        </div>
    </div>

    <div
        v-else
        class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground"
    >
        <MousePointerClick class="h-8 w-8 opacity-30" aria-hidden="true" />
        <p class="text-sm">No clicks recorded yet.</p>
    </div>
</template>
