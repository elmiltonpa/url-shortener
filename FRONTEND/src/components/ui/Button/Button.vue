<script setup lang="ts">
import { useAttrs } from "vue";
import { buttonVariants } from "./button-variants";
import { cn } from "../../../lib/utils";

interface Props {
    variant?:
        | "default"
        | "rust"
        | "outline"
        | "ghost"
        | "link"
        | "destructive"
        | "secondary";
    size?: "default" | "sm" | "lg" | "icon";
    class?: string;
    type?: "button" | "submit" | "reset";
}

const props = withDefaults(defineProps<Props>(), {
    type: "button",
});

const attrs = useAttrs();
</script>

<template>
    <button
        :type="type"
        v-bind="attrs"
        :class="
            cn(
                buttonVariants({
                    variant: props.variant,
                    size: props.size,
                    className: props.class,
                }),
            )
        "
    >
        <slot />
    </button>
</template>

<script lang="ts">
export default {
    inheritAttrs: false,
};
</script>
