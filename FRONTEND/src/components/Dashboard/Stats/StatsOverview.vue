<script setup lang="ts">
import { ref, computed } from "vue";
import {
    Link2,
    Check,
    Copy,
    ExternalLink,
    Calendar,
    Clock,
    MousePointerClick,
} from "lucide-vue-next";
import type { UrlStatsResponse } from "../../../types/url";
import { formatDate, getExpiryBadge } from "../../../utils/formatters";

const props = defineProps<{
    shortCode: string;
    stats: UrlStatsResponse;
}>();

const copied = ref(false);
const copiedShort = ref(false);

const shortUrl = computed(() =>
    props.shortCode ? `${window.location.origin}/${props.shortCode}` : "",
);

const expiryBadge = computed(() => getExpiryBadge(props.stats.expires_at));

function copyLongUrl() {
    if (!props.stats?.original_url) return;
    navigator.clipboard.writeText(props.stats.original_url).then(() => {
        copied.value = true;
        setTimeout(() => (copied.value = false), 1500);
    });
}

function copyShortUrl() {
    if (!props.shortCode) return;
    navigator.clipboard.writeText(shortUrl.value).then(() => {
        copiedShort.value = true;
        setTimeout(() => (copiedShort.value = false), 1500);
    });
}
</script>

<template>
    <div class="space-y-6">
        <div class="rounded-2xl border border-border bg-muted/30 p-4 space-y-3">
            <div class="flex items-center gap-2 min-w-0">
                <Link2 class="h-4 w-4 text-rust shrink-0" />
                <span class="text-sm font-mono text-rust truncate flex-1">{{
                    shortUrl
                }}</span>
                <button
                    @click="copyShortUrl"
                    class="shrink-0 rounded-lg p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
                    :title="copiedShort ? 'Copied!' : 'Copy short URL'"
                    :aria-label="copiedShort ? 'Copied!' : 'Copy short URL'"
                >
                    <Check v-if="copiedShort" class="h-3.5 w-3.5 text-green-400" />
                    <Copy v-else class="h-3.5 w-3.5" />
                </button>
            </div>

            <div class="flex items-start gap-2 min-w-0">
                <ExternalLink
                    class="h-4 w-4 text-muted-foreground shrink-0 mt-0.5"
                />
                <a
                    :href="stats.original_url"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-sm text-muted-foreground hover:text-foreground truncate flex-1 transition-colors"
                >
                    {{ stats.original_url }}
                </a>
                <button
                    @click="copyLongUrl"
                    class="shrink-0 rounded-lg p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
                    :title="copied ? 'Copied!' : 'Copy original URL'"
                    :aria-label="copied ? 'Copied!' : 'Copy original URL'"
                >
                    <Check v-if="copied" class="h-3.5 w-3.5 text-green-400" />
                    <Copy v-else class="h-3.5 w-3.5" />
                </button>
            </div>

            <div class="flex flex-wrap items-center gap-2 pt-1">
                <span
                    class="flex items-center gap-1.5 text-xs text-muted-foreground"
                >
                    <Calendar class="h-3.5 w-3.5" />
                    Created
                    {{ formatDate(stats.created_at) }}
                </span>
                <span
                    v-if="expiryBadge"
                    :class="[
                        'inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full border',
                        expiryBadge.cls,
                    ]"
                >
                    <Clock class="h-3 w-3" />
                    {{ expiryBadge.label }}
                </span>
            </div>
        </div>

        <div
            class="flex items-center gap-4 rounded-2xl border border-rust/20 bg-rust/5 px-5 py-4"
        >
            <div class="rounded-xl bg-rust/15 p-3 text-rust shrink-0">
                <MousePointerClick class="h-6 w-6" />
            </div>
            <div>
                <p class="text-3xl font-extrabold tracking-tight text-foreground">
                    {{ stats.total_clicks.toLocaleString() }}
                </p>
                <p class="text-sm text-muted-foreground font-medium">
                    total clicks
                </p>
            </div>
        </div>
    </div>
</template>
