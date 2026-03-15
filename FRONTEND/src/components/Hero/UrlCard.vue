<script setup lang="ts">
import { ref } from "vue";
import { Check, Copy, BarChart3 } from "lucide-vue-next";
import Button from "../ui/Button/Button.vue";

interface Props {
  shortenedLink: string;
  originalUrl: string;
  shortCode: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: "view-stats", code: string): void;
}>();

const copied = ref(false);

const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(props.shortenedLink);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (err) {
    console.error("Failed to copy!", err);
  }
};
</script>

<template>
  <div
    class="mt-6 glass rounded-xl p-5 border border-primary/20 animate-in fade-in zoom-in-95 duration-300"
    role="region"
    aria-label="Shortened link result"
  >
    <div class="flex items-center justify-between gap-4">
      <div class="min-w-0 flex-1">
        <p class="truncate text-sm font-semibold text-primary">
          <a
            :href="shortenedLink"
            target="_blank"
            rel="noopener noreferrer"
            class="hover:underline"
          >
            {{ shortenedLink }}
          </a>
        </p>
        <p class="truncate text-xs text-muted-foreground" :title="originalUrl">
          {{ originalUrl }}
        </p>
      </div>

      <div class="flex gap-2">
        <Button
          variant="outline"
          size="sm"
          type="button"
          :aria-label="copied ? 'Copied!' : 'Copy link to clipboard'"
          @click="copyToClipboard"
        >
          <Check v-if="copied" class="h-3.5 w-3.5" />
          <Copy v-else class="h-3.5 w-3.5" />
        </Button>

        <Button
          variant="ghost"
          size="sm"
          type="button"
          aria-label="View analytics"
          title="View analytics"
          @click="emit('view-stats', props.shortCode)"
        >
          <BarChart3 class="h-3.5 w-3.5" />
        </Button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.glass {
  background: rgba(var(--background-rgb), 0.8);
  backdrop-filter: blur(8px);
}
</style>
