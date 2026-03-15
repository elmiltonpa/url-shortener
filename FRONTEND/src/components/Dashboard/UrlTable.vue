<script setup lang="ts">
import { ref, computed } from "vue";
import {
    Copy,
    ExternalLink,
    BarChart3,
    Trash2,
    MoreVertical,
    ChevronLeft,
    ChevronRight,
    Check,
    X,
    Loader2,
} from "lucide-vue-next";
import type { UrlData } from "../../types/url";
import { formatDate, getExpiryBadge } from "../../utils/formatters";
import type { ExpiryBadge } from "../../utils/formatters";
import Button from "../ui/Button/Button.vue";

const props = defineProps<{
    urls: UrlData[];
    loading: boolean;
    currentPage: number;
    totalPages: number;
    deletingCodes?: Set<string>;
}>();

const emit = defineEmits<{
    (e: "viewStats", shortCode: string): void;
    (e: "deleteUrl", shortCode: string): void;
    (e: "changePage", page: number): void;
}>();

const copiedCode = ref<string | null>(null);

const copyToClipboard = (text: string, code: string) => {
    navigator.clipboard.writeText(text).then(() => {
        copiedCode.value = code;
        setTimeout(() => (copiedCode.value = null), 1500);
    });
};

const pendingDeleteCode = ref<string | null>(null);

const requestDelete = (shortCode: string) => {
    pendingDeleteCode.value = shortCode;
};

const cancelDelete = () => {
    pendingDeleteCode.value = null;
};

const confirmDelete = (shortCode: string) => {
    pendingDeleteCode.value = null;
    emit("deleteUrl", shortCode);
};

type UrlRowWithBadge = UrlData & { expiryBadge: ExpiryBadge | null };

const urlsWithBadges = computed<UrlRowWithBadge[]>(() =>
    props.urls.map((url) => ({
        ...url,
        expiryBadge: getExpiryBadge(url.expires_at),
    })),
);

const visiblePages = computed(() => {
    const total = props.totalPages;
    const current = props.currentPage;
    if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1);

    const pages: (number | "…")[] = [1];
    if (current > 3) pages.push("…");
    for (
        let p = Math.max(2, current - 1);
        p <= Math.min(total - 1, current + 1);
        p++
    ) {
        pages.push(p);
    }
    if (current < total - 2) pages.push("…");
    pages.push(total);
    return pages;
});
</script>

<template>
    <div class="w-full max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        <!-- Mobile: Card layout -->
        <div class="md:hidden space-y-3">
            <template v-if="loading">
                <div
                    v-for="i in 5"
                    :key="i"
                    class="animate-pulse rounded-xl border border-border bg-card/30 backdrop-blur-md p-4 space-y-3"
                >
                    <div class="h-4 w-3/4 bg-muted rounded" />
                    <div class="h-4 w-1/2 bg-muted rounded" />
                    <div class="flex gap-2">
                        <div class="h-8 w-20 bg-muted rounded" />
                        <div class="h-8 w-20 bg-muted rounded" />
                    </div>
                </div>
            </template>

            <template v-else-if="urls.length === 0">
                <div
                    class="rounded-xl border border-border bg-card/30 backdrop-blur-md p-8 text-center text-muted-foreground italic"
                >
                    No links found. Create your first short link to see it here!
                </div>
            </template>

            <template v-else>
                <div
                    v-for="url in urlsWithBadges"
                    :key="url.short_code"
                    class="rounded-xl border border-border bg-card/30 backdrop-blur-md p-4 space-y-3 transition-colors"
                    :class="{
                        'opacity-60': deletingCodes?.has(url.short_code),
                    }"
                >
                    <!-- Original URL -->
                    <div class="min-w-0">
                        <p
                            class="text-[11px] uppercase tracking-wider text-muted-foreground font-semibold mb-1"
                        >
                            Original URL
                        </p>
                        <p
                            class="text-sm font-medium text-foreground truncate"
                        >
                            {{ url.original_url }}
                        </p>
                    </div>

                    <!-- Short Link + Copy -->
                    <div class="flex items-center gap-2">
                        <code
                            class="text-xs bg-rust/10 text-rust px-2 py-1 rounded font-mono"
                        >
                            /{{ url.short_code }}
                        </code>
                        <button
                            @click="
                                copyToClipboard(
                                    url.short_url,
                                    url.short_code,
                                )
                            "
                            class="text-muted-foreground hover:text-rust transition-colors cursor-pointer"
                            :title="
                                copiedCode === url.short_code
                                    ? 'Copied!'
                                    : 'Copy short link'
                            "
                        >
                            <Check
                                v-if="copiedCode === url.short_code"
                                class="h-3.5 w-3.5 text-emerald-500"
                            />
                            <Copy v-else class="h-3.5 w-3.5" />
                        </button>
                    </div>

                    <!-- Meta row: Clicks + Date + Expiry -->
                    <div class="flex items-center gap-3 flex-wrap text-sm text-muted-foreground">
                        <span
                            class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-rust/10 text-rust"
                        >
                            {{ url.click_count }} clicks
                        </span>
                        <span class="text-xs">{{ formatDate(url.created_at) }}</span>
                        <span
                            v-if="url.expiryBadge"
                            class="inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-semibold border"
                            :class="url.expiryBadge.cls"
                        >
                            {{ url.expiryBadge.label }}
                        </span>
                    </div>

                    <!-- Actions -->
                    <div class="flex items-center gap-1 pt-1 border-t border-border">
                        <Transition
                            enter-active-class="transition duration-150 ease-out"
                            enter-from-class="opacity-0 scale-95"
                            enter-to-class="opacity-100 scale-100"
                            leave-active-class="transition duration-100 ease-in"
                            leave-from-class="opacity-100 scale-100"
                            leave-to-class="opacity-0 scale-95"
                            mode="out-in"
                        >
                            <div
                                v-if="
                                    pendingDeleteCode === url.short_code
                                "
                                key="confirm"
                                class="flex items-center gap-1"
                            >
                                <span
                                    class="text-xs text-muted-foreground mr-1 font-medium"
                                >
                                    Delete?
                                </span>
                                <Button
                                    variant="ghost"
                                    size="icon"
                                    title="Confirm delete"
                                    class="h-8 w-8 text-red-500 hover:text-red-400 hover:bg-red-500/10 cursor-pointer"
                                    :disabled="
                                        deletingCodes?.has(url.short_code)
                                    "
                                    @click="
                                        confirmDelete(url.short_code)
                                    "
                                >
                                    <Loader2
                                        v-if="
                                            deletingCodes?.has(
                                                url.short_code,
                                            )
                                        "
                                        class="h-4 w-4 animate-spin"
                                    />
                                    <Check v-else class="h-4 w-4" />
                                </Button>
                                <Button
                                    variant="ghost"
                                    size="icon"
                                    title="Cancel"
                                    class="h-8 w-8 text-muted-foreground hover:text-foreground cursor-pointer"
                                    @click="cancelDelete"
                                >
                                    <X class="h-4 w-4" />
                                </Button>
                            </div>

                            <div
                                v-else
                                key="actions"
                                class="flex items-center gap-1"
                            >
                                <a
                                    :href="url.short_url"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    title="Open link"
                                >
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        class="h-8 w-8 text-muted-foreground hover:text-foreground cursor-pointer"
                                    >
                                        <ExternalLink class="h-4 w-4" />
                                    </Button>
                                </a>

                                <Button
                                    variant="ghost"
                                    size="icon"
                                    title="View stats"
                                    class="h-8 w-8 text-muted-foreground hover:text-rust cursor-pointer"
                                    @click="
                                        emit(
                                            'viewStats',
                                            url.short_code,
                                        )
                                    "
                                >
                                    <BarChart3 class="h-4 w-4" />
                                </Button>

                                <Button
                                    variant="ghost"
                                    size="icon"
                                    title="Delete link"
                                    class="h-8 w-8 text-muted-foreground hover:text-red-500 cursor-pointer"
                                    :disabled="
                                        deletingCodes?.has(url.short_code)
                                    "
                                    @click="
                                        requestDelete(url.short_code)
                                    "
                                >
                                    <Loader2
                                        v-if="
                                            deletingCodes?.has(
                                                url.short_code,
                                            )
                                        "
                                        class="h-4 w-4 animate-spin"
                                    />
                                    <Trash2 v-else class="h-4 w-4" />
                                </Button>
                            </div>
                        </Transition>
                    </div>
                </div>
            </template>
        </div>

        <!-- Desktop: Table layout -->
        <div
            class="hidden md:block overflow-hidden rounded-2xl border border-border bg-card/30 backdrop-blur-md"
        >
            <div class="overflow-x-auto">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="border-b border-border bg-muted/50">
                            <th
                                class="px-3 sm:px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground"
                            >
                                Original URL
                            </th>
                            <th
                                class="px-3 sm:px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground"
                            >
                                Short Link
                            </th>
                            <th
                                class="px-3 sm:px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground text-center"
                            >
                                Clicks
                            </th>
                            <th
                                class="hidden sm:table-cell px-3 sm:px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground"
                            >
                                Created
                            </th>
                            <th
                                class="px-3 sm:px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground text-right"
                            >
                                Actions
                            </th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-border">
                        <template v-if="loading">
                            <tr v-for="i in 5" :key="i" class="animate-pulse">
                                <td class="px-3 sm:px-6 py-4">
                                    <div class="h-4 w-24 sm:w-48 bg-muted rounded" />
                                </td>
                                <td class="px-3 sm:px-6 py-4">
                                    <div class="h-4 w-20 sm:w-32 bg-muted rounded" />
                                </td>
                                <td class="px-3 sm:px-6 py-4">
                                    <div
                                        class="mx-auto h-4 w-8 bg-muted rounded"
                                    />
                                </td>
                                <td class="hidden sm:table-cell px-3 sm:px-6 py-4">
                                    <div class="h-4 w-24 bg-muted rounded" />
                                </td>
                                <td class="px-3 sm:px-6 py-4">
                                    <div
                                        class="ml-auto h-8 w-24 bg-muted rounded"
                                    />
                                </td>
                            </tr>
                        </template>

                        <template v-else-if="urls.length === 0">
                            <tr>
                                <td
                                    colspan="5"
                                    class="px-3 sm:px-6 py-12 text-center text-muted-foreground italic"
                                >
                                    No links found. Create your first short link
                                    to see it here!
                                </td>
                            </tr>
                        </template>

                        <template v-else>
                            <tr
                                v-for="url in urlsWithBadges"
                                :key="url.short_code"
                                class="group transition-colors hover:bg-muted/30"
                                :class="{
                                    'opacity-60': deletingCodes?.has(
                                        url.short_code,
                                    ),
                                }"
                            >
                                <td class="px-3 sm:px-6 py-4">
                                    <div class="flex flex-col max-w-[120px] sm:max-w-md">
                                        <span
                                            class="text-sm font-medium truncate text-foreground"
                                        >
                                            {{ url.original_url }}
                                        </span>
                                    </div>
                                </td>

                                <td class="px-3 sm:px-6 py-4">
                                    <div class="flex items-center gap-2">
                                        <code
                                            class="text-xs bg-rust/10 text-rust px-2 py-1 rounded font-mono"
                                        >
                                            /{{ url.short_code }}
                                        </code>
                                        <button
                                            @click="
                                                copyToClipboard(
                                                    url.short_url,
                                                    url.short_code,
                                                )
                                            "
                                            class="text-muted-foreground hover:text-rust transition-colors cursor-pointer"
                                            :title="
                                                copiedCode === url.short_code
                                                    ? 'Copied!'
                                                    : 'Copy short link'
                                            "
                                        >
                                            <Check
                                                v-if="
                                                    copiedCode ===
                                                    url.short_code
                                                "
                                                class="h-3.5 w-3.5 text-emerald-500"
                                            />
                                            <Copy v-else class="h-3.5 w-3.5" />
                                        </button>
                                    </div>
                                </td>

                                <td class="px-3 sm:px-6 py-4 text-center">
                                    <span
                                        class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-rust/10 text-rust"
                                    >
                                        {{ url.click_count }}
                                    </span>
                                </td>

                                <td
                                    class="hidden sm:table-cell px-3 sm:px-6 py-4 text-sm text-muted-foreground whitespace-nowrap"
                                >
                                    <div class="flex flex-col gap-1">
                                        <span>{{
                                            formatDate(url.created_at)
                                        }}</span>
                                        <span
                                            v-if="url.expiryBadge"
                                            class="inline-flex self-start items-center px-1.5 py-0.5 rounded text-[10px] font-semibold border"
                                            :class="url.expiryBadge.cls"
                                        >
                                            {{ url.expiryBadge.label }}
                                        </span>
                                    </div>
                                </td>

                                <td
                                    class="px-3 sm:px-6 py-4 text-right whitespace-nowrap"
                                >
                                    <Transition
                                        enter-active-class="transition duration-150 ease-out"
                                        enter-from-class="opacity-0 scale-95"
                                        enter-to-class="opacity-100 scale-100"
                                        leave-active-class="transition duration-100 ease-in"
                                        leave-from-class="opacity-100 scale-100"
                                        leave-to-class="opacity-0 scale-95"
                                        mode="out-in"
                                    >
                                        <div
                                            v-if="
                                                pendingDeleteCode ===
                                                url.short_code
                                            "
                                            key="confirm"
                                            class="flex items-center justify-end gap-1"
                                        >
                                            <span
                                                class="text-xs text-muted-foreground mr-1 font-medium"
                                            >
                                                Delete?
                                            </span>
                                            <Button
                                                variant="ghost"
                                                size="icon"
                                                title="Confirm delete"
                                                class="h-7 w-7 text-red-500 hover:text-red-400 hover:bg-red-500/10 cursor-pointer"
                                                :disabled="
                                                    deletingCodes?.has(
                                                        url.short_code,
                                                    )
                                                "
                                                @click="
                                                    confirmDelete(
                                                        url.short_code,
                                                    )
                                                "
                                            >
                                                <Loader2
                                                    v-if="
                                                        deletingCodes?.has(
                                                            url.short_code,
                                                        )
                                                    "
                                                    class="h-3.5 w-3.5 animate-spin"
                                                />
                                                <Check
                                                    v-else
                                                    class="h-3.5 w-3.5"
                                                />
                                            </Button>
                                            <Button
                                                variant="ghost"
                                                size="icon"
                                                title="Cancel"
                                                class="h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer"
                                                @click="cancelDelete"
                                            >
                                                <X class="h-3.5 w-3.5" />
                                            </Button>
                                        </div>

                                        <div
                                            v-else
                                            key="actions"
                                            class="relative flex items-center justify-end h-8 min-w-30"
                                        >
                                            <div
                                                class="flex items-center gap-1 opacity-100 md:opacity-0 group-hover:opacity-100 transition-all duration-200 translate-x-0 md:translate-x-2 group-hover:translate-x-0"
                                            >
                                                <a
                                                    :href="url.short_url"
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    title="Open link"
                                                >
                                                    <Button
                                                        variant="ghost"
                                                        size="icon"
                                                        class="h-8 w-8 text-muted-foreground hover:text-foreground cursor-pointer"
                                                    >
                                                        <ExternalLink
                                                            class="h-4 w-4"
                                                        />
                                                    </Button>
                                                </a>

                                                <Button
                                                    variant="ghost"
                                                    size="icon"
                                                    title="View stats"
                                                    class="h-8 w-8 text-muted-foreground hover:text-rust cursor-pointer"
                                                    @click="
                                                        emit(
                                                            'viewStats',
                                                            url.short_code,
                                                        )
                                                    "
                                                >
                                                    <BarChart3
                                                        class="h-4 w-4"
                                                    />
                                                </Button>

                                                <Button
                                                    variant="ghost"
                                                    size="icon"
                                                    title="Delete link"
                                                    class="h-8 w-8 text-muted-foreground hover:text-red-500 cursor-pointer"
                                                    :disabled="
                                                        deletingCodes?.has(
                                                            url.short_code,
                                                        )
                                                    "
                                                    @click="
                                                        requestDelete(
                                                            url.short_code,
                                                        )
                                                    "
                                                >
                                                    <Loader2
                                                        v-if="
                                                            deletingCodes?.has(
                                                                url.short_code,
                                                            )
                                                        "
                                                        class="h-4 w-4 animate-spin"
                                                    />
                                                    <Trash2
                                                        v-else
                                                        class="h-4 w-4"
                                                    />
                                                </Button>
                                            </div>

                                            <div
                                                class="hidden md:block absolute right-0 opacity-100 group-hover:opacity-0 transition-opacity duration-200 pointer-events-none"
                                            >
                                                <MoreVertical
                                                    class="h-4 w-4 text-muted-foreground/40"
                                                />
                                            </div>
                                        </div>
                                    </Transition>
                                </td>
                            </tr>
                        </template>
                    </tbody>
                </table>
            </div>
        </div>

        <div
            v-if="!loading && totalPages > 1"
            class="flex items-center justify-between mt-4 px-1"
        >
            <p class="text-xs text-muted-foreground">
                Page {{ currentPage }} of {{ totalPages }}
            </p>

            <div class="flex items-center gap-1">
                <button
                    class="h-8 w-8 flex items-center justify-center rounded-lg border border-border bg-card/50 text-muted-foreground hover:text-foreground hover:bg-muted disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer"
                    :disabled="currentPage <= 1"
                    @click="emit('changePage', currentPage - 1)"
                    aria-label="Previous page"
                >
                    <ChevronLeft class="h-4 w-4" />
                </button>

                <template v-for="p in visiblePages" :key="String(p)">
                    <span
                        v-if="p === '…'"
                        class="h-8 w-8 flex items-center justify-center text-xs text-muted-foreground select-none"
                    >
                        …
                    </span>
                    <button
                        v-else
                        class="h-8 w-8 flex items-center justify-center rounded-lg border text-xs font-medium transition-colors cursor-pointer"
                        :class="
                            p === currentPage
                                ? 'bg-rust text-white border-rust shadow-sm'
                                : 'border-border bg-card/50 text-muted-foreground hover:text-foreground hover:bg-muted'
                        "
                        @click="emit('changePage', p as number)"
                    >
                        {{ p }}
                    </button>
                </template>

                <button
                    class="h-8 w-8 flex items-center justify-center rounded-lg border border-border bg-card/50 text-muted-foreground hover:text-foreground hover:bg-muted disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer"
                    :disabled="currentPage >= totalPages"
                    @click="emit('changePage', currentPage + 1)"
                    aria-label="Next page"
                >
                    <ChevronRight class="h-4 w-4" />
                </button>
            </div>
        </div>
    </div>
</template>
