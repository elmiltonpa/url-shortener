<script setup lang="ts">
import { useAttrs } from "vue";
import { cn } from "../../lib/utils";

interface Props {
    modelValue?: string | number;
    type?: string;
    class?: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
    (e: "update:modelValue", value: string | number): void;
}>();

const attrs = useAttrs();

const handleInput = (event: Event) => {
    const target = event.target as HTMLInputElement;
    emit("update:modelValue", target.value);
};
</script>

<template>
    <div
        :class="
            cn(
                'group relative flex items-center rounded-xl border border-border bg-card p-1 transition-all focus-within:border-primary/50 focus-within:ring-1 focus-within:ring-primary/20',
                props.class,
            )
        "
    >
        <div
            v-if="$slots.icon"
            class="ml-3 flex items-center justify-center text-muted-foreground group-focus-within:text-primary transition-colors"
        >
            <slot name="icon" />
        </div>

        <input
            v-bind="attrs"
            :type="type || 'text'"
            :value="modelValue"
            @input="handleInput"
            :class="
                cn(
                    'flex h-10 w-full bg-transparent py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50',
                    $slots.icon ? 'pl-2 pr-3' : 'px-3',
                )
            "
        />
    </div>
</template>

<script lang="ts">
export default {
    inheritAttrs: false,
};
</script>
