<script setup lang="ts">
import { useAttrs } from "vue";
import { buttonVariants } from "./button-variants";
import { cn } from "../../../lib/utils";

interface Props {
    href: string;
    variant?:
        | "default"
        | "outline"
        | "ghost"
        | "link"
        | "destructive"
        | "secondary";
    size?: "default" | "sm" | "lg" | "icon";
    class?: string;
}

const props = defineProps<Props>();
const attrs = useAttrs();
</script>

<template>
    <a
        :href="href"
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
    </a>
</template>

<script lang="ts">
export default {
    inheritAttrs: false,
};
</script>
