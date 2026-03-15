<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { Loader2, AlertCircle } from "lucide-vue-next";
import { actions } from "astro:actions";
import type { UrlStatsResponse } from "../../types/url";

import StatsHeader from "./Stats/StatsHeader.vue";
import StatsOverview from "./Stats/StatsOverview.vue";
import StatsCharts from "./Stats/StatsCharts.vue";
import StatsHistory from "./Stats/StatsHistory.vue";

const props = defineProps<{
  shortCode: string | null;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const stats = ref<UrlStatsResponse | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);
const isMounted = ref(false);

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
    const header = document.querySelector("header.fixed");
    if (locked) {
      const scrollbarWidth =
        window.innerWidth - document.documentElement.clientWidth;
      const pad = `${scrollbarWidth}px`;
      document.body.style.paddingRight = pad;
      document.body.style.overflow = "hidden";
      if (header instanceof HTMLElement) header.style.paddingRight = pad;
    } else {
      document.body.style.paddingRight = "";
      document.body.style.overflow = "";
      if (header instanceof HTMLElement) header.style.paddingRight = "";
    }
  }
}

function handleEscKey(e: KeyboardEvent) {
  if (e.key === "Escape") emit("close");
}

onMounted(() => {
  isMounted.value = true;
  document.addEventListener("keydown", handleEscKey);
});
onUnmounted(() => {
  document.removeEventListener("keydown", handleEscKey);
  setBodyScroll(false);
});

watch(
  () => props.shortCode,
  async (code) => {
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
</script>

<template>
  <Teleport to="body" :disabled="!isMounted">
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
            <StatsHeader
              :short-code="shortCode"
              :stats="stats"
              :loading="loading"
              @refresh="fetchStats(shortCode!)"
              @close="emit('close')"
            />

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
                <div class="rounded-2xl bg-red-500/10 p-3 text-red-500">
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
                <StatsOverview :short-code="shortCode" :stats="stats" />
                <StatsCharts :clicks="stats.stats" />
                <StatsHistory :clicks="stats.stats" />
              </template>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
