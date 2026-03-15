<script setup lang="ts">
import { computed } from "vue";
import { Monitor, Smartphone, Globe, TrendingUp } from "lucide-vue-next";
import type { StatEntry } from "../../../types/url";
import { parseBrowser, parseDevice, cleanReferrer } from "../../../utils/analytics";

const props = defineProps<{
    clicks: StatEntry[];
}>();

const topReferrers = computed(() => {
    if (!props.clicks.length) return [];
    const counts = new Map<string, number>();
    for (const click of props.clicks) {
        const key = cleanReferrer(click.referrer);
        counts.set(key, (counts.get(key) ?? 0) + 1);
    }
    const total = props.clicks.length;
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
    if (!props.clicks.length) return [];
    const counts = new Map<string, number>();
    for (const click of props.clicks) {
        const b = parseBrowser(click.user_agent);
        counts.set(b, (counts.get(b) ?? 0) + 1);
    }
    const total = props.clicks.length;
    return [...counts.entries()]
        .sort((a, b) => b[1] - a[1])
        .map(([label, count]) => ({
            label,
            count,
            pct: Math.round((count / total) * 100),
        }));
});

const deviceBreakdown = computed(() => {
    if (!props.clicks.length)
        return { mobile: 0, desktop: 0, mobilePct: 0, desktopPct: 0 };
    let mobile = 0;
    for (const click of props.clicks) {
        if (parseDevice(click.user_agent) === "Mobile") mobile++;
    }
    const total = props.clicks.length;
    const desktop = total - mobile;
    return {
        mobile,
        desktop,
        mobilePct: Math.round((mobile / total) * 100),
        desktopPct: Math.round((desktop / total) * 100),
    };
});
</script>

<template>
    <div class="space-y-6">
        <div v-if="clicks.length" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
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
                        <div class="flex items-center justify-between text-sm">
                            <span
                                class="flex items-center gap-1.5 text-foreground"
                            >
                                <Monitor
                                    class="h-3.5 w-3.5 text-muted-foreground"
                                />
                                Desktop
                            </span>
                            <span class="font-semibold">
                                {{ deviceBreakdown.desktop }}
                                <span
                                    class="text-xs text-muted-foreground font-normal ml-1"
                                >
                                    ({{ deviceBreakdown.desktopPct }}%)
                                </span>
                            </span>
                        </div>
                        <div
                            class="h-1.5 rounded-full bg-muted overflow-hidden"
                        >
                            <div
                                class="h-full rounded-full bg-rust/60 transition-all duration-500"
                                :style="{
                                    width: deviceBreakdown.desktopPct + '%',
                                }"
                            />
                        </div>
                    </div>
                    <div class="space-y-1">
                        <div class="flex items-center justify-between text-sm">
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
                                    ({{ deviceBreakdown.mobilePct }}%)
                                </span>
                            </span>
                        </div>
                        <div
                            class="h-1.5 rounded-full bg-muted overflow-hidden"
                        >
                            <div
                                class="h-full rounded-full bg-rust/40 transition-all duration-500"
                                :style="{
                                    width: deviceBreakdown.mobilePct + '%',
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
                        <div class="flex items-center justify-between text-sm">
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
                                    width: browser.pct + '%',
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
                <TrendingUp class="h-3.5 w-3.5" /> Top Referrers
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
                        <div class="flex items-center justify-between text-sm">
                            <span class="truncate text-foreground">{{
                                referrer.label
                            }}</span>
                            <span class="font-semibold ml-2 shrink-0">
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
                                    width: referrer.pct + '%',
                                }"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
