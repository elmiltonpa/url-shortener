<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from "vue";
import {
    X,
    ExternalLink,
    Copy,
    Check,
    MousePointerClick,
    Globe,
    Monitor,
    Smartphone,
    Clock,
    TrendingUp,
    BarChart3,
    Loader2,
    AlertCircle,
    Link2,
    ChevronLeft,
    ChevronRight,
    Calendar,
    RefreshCw,
} from "lucide-vue-next";
import { actions } from "astro:actions";
import type { UrlStatsResponse } from "../../types/url";
import {
    formatDate,
    formatDateTime,
    getExpiryBadge,
} from "../../utils/formatters";
import {
    parseBrowser,
    parseDevice,
    cleanReferrer,
} from "../../utils/analytics";

const props = defineProps<{
    shortCode: string | null;
}>();

const emit = defineEmits<{
    (e: "close"): void;
}>();

const stats = ref<UrlStatsResponse | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);
const copied = ref(false);
const copiedShort = ref(false);

const currentPage = ref(1);
const PER_PAGE = 10;

async function fetchStats(code: string) {
    loading.value = true;
    error.value = null;
    try {
        const { data, error: err } = await actions.getUrlStats({
            code,
            page: 1,
            per_page: 50,
        });
        if (err) {
            error.value = err.message || "Failed to load stats.";
        } else {
            stats.value = data as UrlStatsResponse;
        }
    } catch {
        error.value = "A connection error occurred.";
    } finally {
        loading.value = false;
    }
}

function setBodyScroll(locked: boolean) {
    if (typeof document !== "undefined") {
        document.body.style.overflow = locked ? "hidden" : "";
    }
}

function handleEscKey(e: KeyboardEvent) {
    if (e.key === "Escape") emit("close");
}

onMounted(() => document.addEventListener("keydown", handleEscKey));
onUnmounted(() => {
    document.removeEventListener("keydown", handleEscKey);
    setBodyScroll(false);
});

watch(
    () => props.shortCode,
    async (code) => {
        currentPage.value = 1;
        if (!code) {
            stats.value = null;
            error.value = null;
            setBodyScroll(false);
            return;
        }
        setBodyScroll(true);
        await fetchStats(code);
    },
    { immediate: true },
);

function copyLongUrl() {
    if (!stats.value?.original_url) return;
    navigator.clipboard.writeText(stats.value.original_url).then(() => {
        copied.value = true;
        setTimeout(() => (copied.value = false), 1500);
    });
}

function copyShortUrl() {
    if (!props.shortCode) return;
    const url = `${window.location.origin}/${props.shortCode}`;
    navigator.clipboard.writeText(url).then(() => {
        copiedShort.value = true;
        setTimeout(() => (copiedShort.value = false), 1500);
    });
}

const expiryBadge = computed(() => getExpiryBadge(stats.value?.expires_at));

const topReferrers = computed(() => {
    if (!stats.value?.stats.length) return [];
    const counts = new Map<string, number>();
    for (const click of stats.value.stats) {
        const key = cleanReferrer(click.referrer);
        counts.set(key, (counts.get(key) ?? 0) + 1);
    }
    const total = stats.value.stats.length;
    return [...counts.entries()]
        .sort((a, b) => b[1] - a[1])
        .slice(0, 5)
        .map(([label, count]) => ({
            label,
            count,
            pct: Math.round((count / total) * 100),
        }));
});

const browserBreakdown = computed(() => {
    if (!stats.value?.stats.length) return [];
    const counts = new Map<string, number>();
    for (const click of stats.value.stats) {
        const b = parseBrowser(click.user_agent);
        counts.set(b, (counts.get(b) ?? 0) + 1);
    }
    const total = stats.value.stats.length;
    return [...counts.entries()]
        .sort((a, b) => b[1] - a[1])
        .map(([label, count]) => ({
            label,
            count,
            pct: Math.round((count / total) * 100),
        }));
});

const deviceBreakdown = computed(() => {
    if (!stats.value?.stats.length)
        return { mobile: 0, desktop: 0, mobilePct: 0, desktopPct: 0 };
    let mobile = 0;
    for (const click of stats.value.stats) {
        if (parseDevice(click.user_agent) === "Mobile") mobile++;
    }
    const total = stats.value.stats.length;
    const desktop = total - mobile;
    return {
        mobile,
        desktop,
        mobilePct: Math.round((mobile / total) * 100),
        desktopPct: Math.round((desktop / total) * 100),
    };
});

const totalPages = computed(() =>
    stats.value
        ? Math.max(1, Math.ceil(stats.value.stats.length / PER_PAGE))
        : 1,
);

const paginatedClicks = computed(() => {
    if (!stats.value?.stats.length) return [];
    const start = (currentPage.value - 1) * PER_PAGE;
    return stats.value.stats.slice(start, start + PER_PAGE);
});

function prevPage() {
    if (currentPage.value > 1) currentPage.value--;
}
function nextPage() {
    if (currentPage.value < totalPages.value) currentPage.value++;
}

const shortUrl = computed(() =>
    props.shortCode ? `${window.location.origin}/${props.shortCode}` : "",
);
</script>

<template>
    <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition duration-150 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
    >
        <div
            v-if="shortCode"
            class="fixed inset-0 z-1000 flex items-center justify-center p-4"
        >
            <div
                class="absolute inset-0 bg-black/60 backdrop-blur-sm"
                @click="emit('close')"
                aria-hidden="true"
            />

            <Transition
                enter-active-class="transition duration-200 ease-out"
                enter-from-class="opacity-0 scale-95 translate-y-4"
                enter-to-class="opacity-100 scale-100 translate-y-0"
                leave-active-class="transition duration-150 ease-in"
                leave-from-class="opacity-100 scale-100 translate-y-0"
                leave-to-class="opacity-0 scale-95 translate-y-2"
                appear
            >
                <div
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby="stats-modal-title"
                    class="relative z-10 w-full max-w-3xl max-h-[90vh] flex flex-col rounded-2xl border border-border bg-card shadow-2xl overflow-hidden"
                >
                    <div
                        class="flex items-center justify-between gap-4 px-6 py-5 border-b border-border shrink-0"
                    >
                        <div class="flex items-center gap-3 min-w-0">
                            <div
                                class="rounded-xl bg-rust/10 p-2.5 text-rust shrink-0"
                            >
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
                                @click="fetchStats(shortCode!)"
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

                    <div class="flex-1 overflow-y-auto p-6 space-y-6">
                        <div
                            v-if="loading"
                            class="flex flex-col items-center justify-center py-20 gap-3 text-muted-foreground"
                            aria-live="polite"
                            aria-busy="true"
                        >
                            <Loader2 class="h-8 w-8 animate-spin text-rust" />
                            <p class="text-sm font-medium">Loading stats…</p>
                        </div>

                        <div
                            v-else-if="error"
                            class="flex flex-col items-center justify-center py-20 gap-4"
                            role="alert"
                        >
                            <div
                                class="rounded-2xl bg-red-500/10 p-3 text-red-500"
                            >
                                <AlertCircle class="h-7 w-7" />
                            </div>
                            <p class="text-sm font-medium text-red-400">
                                {{ error }}
                            </p>
                            <button
                                @click="fetchStats(shortCode!)"
                                class="text-xs text-muted-foreground hover:text-foreground underline underline-offset-2 transition-colors cursor-pointer"
                            >
                                Try again
                            </button>
                        </div>

                        <template v-else-if="stats">
                            <div
                                class="rounded-2xl border border-border bg-muted/30 p-4 space-y-3"
                            >
                                <div class="flex items-center gap-2 min-w-0">
                                    <Link2 class="h-4 w-4 text-rust shrink-0" />
                                    <span
                                        class="text-sm font-mono text-rust truncate flex-1"
                                        >{{ shortUrl }}</span
                                    >
                                    <button
                                        @click="copyShortUrl"
                                        class="shrink-0 rounded-lg p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
                                        :title="
                                            copiedShort
                                                ? 'Copied!'
                                                : 'Copy short URL'
                                        "
                                        :aria-label="
                                            copiedShort
                                                ? 'Copied!'
                                                : 'Copy short URL'
                                        "
                                    >
                                        <Check
                                            v-if="copiedShort"
                                            class="h-3.5 w-3.5 text-green-400"
                                        />
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
                                        :title="
                                            copied
                                                ? 'Copied!'
                                                : 'Copy original URL'
                                        "
                                        :aria-label="
                                            copied
                                                ? 'Copied!'
                                                : 'Copy original URL'
                                        "
                                    >
                                        <Check
                                            v-if="copied"
                                            class="h-3.5 w-3.5 text-green-400"
                                        />
                                        <Copy v-else class="h-3.5 w-3.5" />
                                    </button>
                                </div>

                                <div
                                    class="flex flex-wrap items-center gap-2 pt-1"
                                >
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
                                <div
                                    class="rounded-xl bg-rust/15 p-3 text-rust shrink-0"
                                >
                                    <MousePointerClick class="h-6 w-6" />
                                </div>
                                <div>
                                    <p
                                        class="text-3xl font-extrabold tracking-tight text-foreground"
                                    >
                                        {{
                                            stats.total_clicks.toLocaleString()
                                        }}
                                    </p>
                                    <p
                                        class="text-sm text-muted-foreground font-medium"
                                    >
                                        total clicks
                                    </p>
                                </div>
                            </div>

                            <div
                                v-if="stats.stats.length"
                                class="grid grid-cols-1 sm:grid-cols-2 gap-4"
                            >
                                <div
                                    class="rounded-2xl border border-border bg-muted/20 p-4 space-y-3"
                                >
                                    <p
                                        class="text-xs font-semibold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5"
                                    >
                                        <Monitor class="h-3.5 w-3.5" />
                                        Devices
                                    </p>
                                    <div class="space-y-2">
                                        <div class="space-y-1">
                                            <div
                                                class="flex items-center justify-between text-sm"
                                            >
                                                <span
                                                    class="flex items-center gap-1.5 text-foreground"
                                                >
                                                    <Monitor
                                                        class="h-3.5 w-3.5 text-muted-foreground"
                                                    />
                                                    Desktop
                                                </span>
                                                <span class="font-semibold">
                                                    {{
                                                        deviceBreakdown.desktop
                                                    }}
                                                    <span
                                                        class="text-xs text-muted-foreground font-normal ml-1"
                                                    >
                                                        ({{
                                                            deviceBreakdown.desktopPct
                                                        }}%)
                                                    </span>
                                                </span>
                                            </div>
                                            <div
                                                class="h-1.5 rounded-full bg-muted overflow-hidden"
                                            >
                                                <div
                                                    class="h-full rounded-full bg-rust/60 transition-all duration-500"
                                                    :style="{
                                                        width:
                                                            deviceBreakdown.desktopPct +
                                                            '%',
                                                    }"
                                                />
                                            </div>
                                        </div>
                                        <div class="space-y-1">
                                            <div
                                                class="flex items-center justify-between text-sm"
                                            >
                                                <span
                                                    class="flex items-center gap-1.5 text-foreground"
                                                >
                                                    <Smartphone
                                                        class="h-3.5 w-3.5 text-muted-foreground"
                                                    />
                                                    Mobile
                                                </span>
                                                <span class="font-semibold">
                                                    {{ deviceBreakdown.mobile }}
                                                    <span
                                                        class="text-xs text-muted-foreground font-normal ml-1"
                                                    >
                                                        ({{
                                                            deviceBreakdown.mobilePct
                                                        }}%)
                                                    </span>
                                                </span>
                                            </div>
                                            <div
                                                class="h-1.5 rounded-full bg-muted overflow-hidden"
                                            >
                                                <div
                                                    class="h-full rounded-full bg-rust/40 transition-all duration-500"
                                                    :style="{
                                                        width:
                                                            deviceBreakdown.mobilePct +
                                                            '%',
                                                    }"
                                                />
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div
                                    class="rounded-2xl border border-border bg-muted/20 p-4 space-y-3"
                                >
                                    <p
                                        class="text-xs font-semibold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5"
                                    >
                                        <Globe class="h-3.5 w-3.5" /> Browsers
                                    </p>
                                    <div class="space-y-2">
                                        <div
                                            v-for="browser in browserBreakdown"
                                            :key="browser.label"
                                            class="space-y-1"
                                        >
                                            <div
                                                class="flex items-center justify-between text-sm"
                                            >
                                                <span class="text-foreground">{{
                                                    browser.label
                                                }}</span>
                                                <span class="font-semibold">
                                                    {{ browser.count }}
                                                    <span
                                                        class="text-xs text-muted-foreground font-normal ml-1"
                                                    >
                                                        ({{ browser.pct }}%)
                                                    </span>
                                                </span>
                                            </div>
                                            <div
                                                class="h-1.5 rounded-full bg-muted overflow-hidden"
                                            >
                                                <div
                                                    class="h-full rounded-full bg-rust/50 transition-all duration-500"
                                                    :style="{
                                                        width:
                                                            browser.pct + '%',
                                                    }"
                                                />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div
                                v-if="topReferrers.length"
                                class="rounded-2xl border border-border bg-muted/20 p-4 space-y-3"
                            >
                                <p
                                    class="text-xs font-semibold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5"
                                >
                                    <TrendingUp class="h-3.5 w-3.5" /> Top
                                    Referrers
                                </p>
                                <div class="space-y-2">
                                    <div
                                        v-for="(referrer, i) in topReferrers"
                                        :key="referrer.label"
                                        class="flex items-center gap-3"
                                    >
                                        <span
                                            class="text-xs font-bold text-muted-foreground w-4 text-right shrink-0"
                                        >
                                            {{ i + 1 }}
                                        </span>
                                        <div class="flex-1 space-y-1 min-w-0">
                                            <div
                                                class="flex items-center justify-between text-sm"
                                            >
                                                <span
                                                    class="truncate text-foreground"
                                                    >{{ referrer.label }}</span
                                                >
                                                <span
                                                    class="font-semibold ml-2 shrink-0"
                                                >
                                                    {{ referrer.count }}
                                                    <span
                                                        class="text-xs text-muted-foreground font-normal ml-1"
                                                    >
                                                        ({{ referrer.pct }}%)
                                                    </span>
                                                </span>
                                            </div>
                                            <div
                                                class="h-1.5 rounded-full bg-muted overflow-hidden"
                                            >
                                                <div
                                                    class="h-full rounded-full bg-rust/50 transition-all duration-500"
                                                    :style="{
                                                        width:
                                                            referrer.pct + '%',
                                                    }"
                                                />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div
                                v-if="stats.stats.length"
                                class="rounded-2xl border border-border bg-muted/20 overflow-hidden"
                            >
                                <div
                                    class="flex items-center justify-between px-4 py-3 border-b border-border"
                                >
                                    <p
                                        class="text-xs font-semibold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5"
                                    >
                                        <Clock class="h-3.5 w-3.5" /> Recent
                                        Clicks
                                    </p>
                                    <span class="text-xs text-muted-foreground">
                                        {{ stats.stats.length }} total
                                    </span>
                                </div>

                                <div
                                    class="divide-y divide-border"
                                    role="list"
                                    aria-label="Click history"
                                >
                                    <div
                                        v-for="click in paginatedClicks"
                                        :key="click.id"
                                        class="flex items-center gap-3 px-4 py-2.5 hover:bg-muted/30 transition-colors"
                                        role="listitem"
                                    >
                                        <component
                                            :is="
                                                parseDevice(
                                                    click.user_agent,
                                                ) === 'Mobile'
                                                    ? Smartphone
                                                    : Monitor
                                            "
                                            class="h-3.5 w-3.5 text-muted-foreground shrink-0"
                                            aria-hidden="true"
                                        />

                                        <div class="flex-1 min-w-0">
                                            <p
                                                class="text-sm text-foreground truncate"
                                            >
                                                {{
                                                    cleanReferrer(
                                                        click.referrer,
                                                    )
                                                }}
                                            </p>
                                            <p
                                                class="text-xs text-muted-foreground"
                                            >
                                                {{
                                                    parseBrowser(
                                                        click.user_agent,
                                                    )
                                                }}
                                                <template
                                                    v-if="click.country_code"
                                                >
                                                    ·
                                                    {{
                                                        click.country_code
                                                    }}</template
                                                >
                                            </p>
                                        </div>

                                        <time
                                            class="text-xs text-muted-foreground shrink-0 tabular-nums"
                                            :datetime="click.created_at"
                                        >
                                            {{
                                                formatDateTime(click.created_at)
                                            }}
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
                                    <span
                                        class="text-xs text-muted-foreground"
                                        aria-live="polite"
                                    >
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
                                <MousePointerClick
                                    class="h-8 w-8 opacity-30"
                                    aria-hidden="true"
                                />
                                <p class="text-sm">No clicks recorded yet.</p>
                            </div>
                        </template>
                    </div>
                </div>
            </Transition>
        </div>
    </Transition>
</template>
