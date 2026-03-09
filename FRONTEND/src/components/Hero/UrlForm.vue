<script setup lang="ts">
import { ref, reactive } from "vue";
import { Link2, ArrowRight } from "lucide-vue-next";
import Button from "../ui/Button/Button.vue";
import { actions, isActionError, isInputError } from "astro:actions";
import { useUrlHistory } from "../../composables/useUrlHistory.ts";
import type { UrlData } from "../../types/url";

const props = defineProps<{
    isLoggedIn: boolean;
}>();

const emit = defineEmits<{
    (e: "linkCreated", link: UrlData | null): void;
}>();

interface ActionState {
    error: string | null;
    data: UrlData | null;
}

const url = ref("");
const isPending = ref(false);
const state = reactive<ActionState>({
    error: null,
    data: null,
});

const { addUrl } = useUrlHistory();

const handleSubmit = async () => {
    if (!url.value.trim() || isPending.value) return;

    isPending.value = true;
    state.error = null;
    state.data = null;

    try {
        const { data, error } = await actions.shortenUrl({ url: url.value });

        if (error) {
            state.error = isInputError(error)
                ? (error.fields.url?.[0] ?? "Invalid input")
                : isActionError(error)
                  ? error.message
                  : "An unexpected error occurred";
        } else if (data) {
            if (!props.isLoggedIn) {
                addUrl(data);
            }

            emit("linkCreated", data);

            state.data = data;
            url.value = "";
        }
    } catch (e) {
        state.error = "Failed to connect to the server";
    } finally {
        isPending.value = false;
    }
};
</script>

<template>
    <div class="w-full max-w-xl">
        <form
            @submit.prevent="handleSubmit"
            class="group relative flex items-center rounded-xl border border-border bg-card p-1.5 transition-all focus-within:border-primary/50"
        >
            <label for="url-input" class="sr-only">
                Enter the long URL to shorten
            </label>

            <Link2
                class="ml-3 h-4 w-4 shrink-0 text-muted-foreground"
                aria-hidden="true"
            />

            <input
                id="url-input"
                v-model="url"
                type="text"
                placeholder="Paste your long URL here..."
                required
                class="flex-1 bg-transparent px-3 py-2.5 text-sm focus:outline-none"
                :aria-invalid="!!state.error"
                :aria-describedby="state.error ? 'url-error' : undefined"
            />

            <Button
                type="submit"
                variant="rust"
                class="cursor-pointer"
                :disabled="isPending || !url.trim()"
            >
                <template v-if="isPending">
                    <span class="animate-pulse">Shortening...</span>
                </template>
                <template v-else>
                    <span class="hidden sm:inline">Shorten</span>
                    <ArrowRight class="h-3.5 w-3.5 ml-2" aria-hidden="true" />
                </template>
            </Button>
        </form>

        <transition
            enter-active-class="transition duration-200 ease-out"
            enter-from-class="transform -translate-y-2 opacity-0"
            enter-to-class="transform translate-y-0 opacity-100"
            leave-active-class="transition duration-150 ease-in"
            leave-from-class="opacity-100"
            leave-to-class="opacity-0"
        >
            <p
                v-if="state.error"
                id="url-error"
                role="alert"
                class="mt-2 text-sm text-red-500 font-medium"
            >
                {{ state.error }}
            </p>
        </transition>
    </div>
</template>
