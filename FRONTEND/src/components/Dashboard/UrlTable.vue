<script setup lang="ts">
import {
    Copy,
    ExternalLink,
    BarChart3,
    Trash2,
    MoreVertical,
} from "lucide-vue-next";
import type { UrlData } from "../../types/url";
import Button from "../ui/Button/Button.vue";

defineProps<{
    urls: UrlData[];
    loading: boolean;
}>();

const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
};

const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString(undefined, {
        month: "short",
        day: "numeric",
        year: "numeric",
    });
};
</script>

<template>
    <div class="w-full max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        <div
            class="overflow-hidden rounded-2xl border border-border bg-card/30 backdrop-blur-md"
        >
            <div class="overflow-x-auto">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="border-b border-border bg-muted/50">
                            <th
                                class="px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground"
                            >
                                Original URL
                            </th>
                            <th
                                class="px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground"
                            >
                                Short Link
                            </th>
                            <th
                                class="px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground text-center"
                            >
                                Clicks
                            </th>
                            <th
                                class="px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground"
                            >
                                Created
                            </th>
                            <th
                                class="px-6 py-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground text-right"
                            >
                                Actions
                            </th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-border">
                        <template v-if="loading">
                            <tr v-for="i in 5" :key="i" class="animate-pulse">
                                <td class="px-6 py-4">
                                    <div
                                        class="h-4 w-48 bg-muted rounded"
                                    ></div>
                                </td>
                                <td class="px-6 py-4">
                                    <div
                                        class="h-4 w-32 bg-muted rounded"
                                    ></div>
                                </td>
                                <td class="px-6 py-4">
                                    <div
                                        class="mx-auto h-4 w-8 bg-muted rounded"
                                    ></div>
                                </td>
                                <td class="px-6 py-4">
                                    <div
                                        class="h-4 w-24 bg-muted rounded"
                                    ></div>
                                </td>
                                <td class="px-6 py-4">
                                    <div
                                        class="ml-auto h-8 w-24 bg-muted rounded"
                                    ></div>
                                </td>
                            </tr>
                        </template>
                        <template v-else-if="urls.length === 0">
                            <tr>
                                <td
                                    colspan="5"
                                    class="px-6 py-12 text-center text-muted-foreground italic"
                                >
                                    No links found. Create your first short link
                                    to see it here!
                                </td>
                            </tr>
                        </template>
                        <template v-else>
                            <tr
                                v-for="url in urls"
                                :key="url.short_code"
                                class="group transition-colors hover:bg-muted/30"
                            >
                                <td class="px-6 py-4">
                                    <div class="flex flex-col max-w-md">
                                        <span
                                            class="text-sm font-medium truncate text-foreground"
                                            >{{ url.original_url }}</span
                                        >
                                    </div>
                                </td>
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-2">
                                        <code
                                            class="text-xs bg-rust/10 text-rust px-2 py-1 rounded font-mono"
                                            >/{{ url.short_code }}</code
                                        >
                                        <button
                                            @click="
                                                copyToClipboard(url.short_url)
                                            "
                                            class="text-muted-foreground hover:text-rust transition-colors cursor-pointer"
                                            title="Copy short link"
                                        >
                                            <Copy class="h-3.5 w-3.5" />
                                        </button>
                                    </div>
                                </td>
                                <td class="px-6 py-4 text-center">
                                    <span
                                        class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-rust/10 text-rust"
                                    >
                                        {{ url.click_count }}
                                    </span>
                                </td>
                                <td
                                    class="px-6 py-4 text-sm text-muted-foreground whitespace-nowrap"
                                >
                                    {{ formatDate(url.created_at) }}
                                </td>
                                <td
                                    class="px-6 py-4 text-right whitespace-nowrap"
                                >
                                    <div
                                        class="relative flex items-center justify-end h-8 min-w-30"
                                    >
                                        <div
                                            class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-all duration-200 translate-x-2 group-hover:translate-x-0"
                                        >
                                            <a
                                                :href="url.short_url"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                title="Open Link"
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
                                                title="View Stats"
                                                class="h-8 w-8 text-muted-foreground hover:text-rust cursor-pointer"
                                            >
                                                <BarChart3 class="h-4 w-4" />
                                            </Button>

                                            <Button
                                                variant="ghost"
                                                size="icon"
                                                title="Delete (Coming Soon)"
                                                class="h-8 w-8 text-muted-foreground/30 cursor-not-allowed"
                                            >
                                                <Trash2 class="h-4 w-4" />
                                            </Button>
                                        </div>

                                        <div
                                            class="absolute right-0 opacity-100 group-hover:opacity-0 transition-opacity duration-200 pointer-events-none"
                                        >
                                            <MoreVertical
                                                class="h-4 w-4 text-muted-foreground/40"
                                            />
                                        </div>
                                    </div>
                                </td>
                            </tr>
                        </template>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>
