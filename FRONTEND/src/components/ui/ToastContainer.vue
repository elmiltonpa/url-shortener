<script setup lang="ts">
import { CheckCircle, XCircle, Info, X } from "lucide-vue-next";
import { useToast } from "../../composables/useToast";

const { toasts, dismiss } = useToast();
</script>

<template>
    <div
        class="fixed bottom-5 right-5 z-9999 flex flex-col gap-3 pointer-events-none"
        aria-live="polite"
    >
        <TransitionGroup
            enter-active-class="transition duration-300 ease-out"
            enter-from-class="opacity-0 translate-y-4 scale-95"
            enter-to-class="opacity-100 translate-y-0 scale-100"
            leave-active-class="transition duration-200 ease-in"
            leave-from-class="opacity-100 translate-y-0 scale-100"
            leave-to-class="opacity-0 translate-y-2 scale-95"
            move-class="transition duration-300 ease-in-out"
        >
            <div
                v-for="toast in toasts"
                :key="toast.id"
                class="pointer-events-auto flex items-start gap-3 min-w-72 max-w-sm w-full rounded-2xl border px-4 py-3.5 shadow-xl backdrop-blur-md"
                :class="{
                    'bg-emerald-500/10 border-emerald-500/25 text-emerald-400':
                        toast.type === 'success',
                    'bg-red-500/10 border-red-500/25 text-red-400':
                        toast.type === 'error',
                    'bg-blue-500/10 border-blue-500/25 text-blue-400':
                        toast.type === 'info',
                }"
            >
                <div class="shrink-0 mt-0.5">
                    <CheckCircle
                        v-if="toast.type === 'success'"
                        class="h-5 w-5"
                    />
                    <XCircle
                        v-else-if="toast.type === 'error'"
                        class="h-5 w-5"
                    />
                    <Info v-else class="h-5 w-5" />
                </div>
                <p class="flex-1 text-sm font-medium leading-snug">
                    {{ toast.message }}
                </p>
                <button
                    @click="dismiss(toast.id)"
                    class="shrink-0 mt-0.5 opacity-60 hover:opacity-100 transition-opacity cursor-pointer"
                >
                    <X class="h-4 w-4" />
                </button>
            </div>
        </TransitionGroup>
    </div>
</template>
