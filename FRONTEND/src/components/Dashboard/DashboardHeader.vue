<script setup lang="ts">
import { RefreshCcw } from "lucide-vue-next";

defineProps<{
    loading: boolean;
}>();

const emit = defineEmits<{
    (e: "refresh"): void;
}>();
</script>

<template>
    <div class="w-full max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 mb-10">
        <div
            class="flex flex-col md:flex-row md:items-center justify-between gap-6"
        >
            <div>
                <h1
                    class="text-4xl font-extrabold tracking-tight text-foreground"
                >
                    Dashboard
                </h1>
                <p class="text-muted-foreground mt-2 text-lg">
                    Manage and track your shortened links performance
                </p>
            </div>

            <div class="flex items-center gap-3">
                <button
                    @click="emit('refresh')"
                    class="flex items-center gap-2 px-4 py-2 rounded-xl bg-muted/50 border border-border hover:bg-muted transition-colors text-sm font-medium cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                    :disabled="loading"
                >
                    <RefreshCcw
                        class="h-4 w-4"
                        :class="{ 'animate-spin': loading }"
                    />
                    Refresh
                </button>
            </div>
        </div>
    </div>
</template>
