<script setup lang="ts">
import { cn } from "../../lib/utils";

interface LogoProps {
    class?: string;
    size?: "sm" | "md" | "lg" | "xl";
}

const props = withDefaults(defineProps<LogoProps>(), {
    size: "md",
});

const sizeClasses = {
    sm: "text-lg",
    md: "text-xl",
    lg: "text-2xl",
    xl: "text-3xl",
};
</script>

<template>
    <a
        href="/"
        :class="
            cn(
                'group flex items-center gap-2.5 transition-opacity hover:opacity-90',
                props.class,
            )
        "
    >
        <span
            :class="
                cn(
                    'font-bold tracking-tight text-foreground italic',
                    sizeClasses[props.size],
                )
            "
        >
            Short<span class="text-rust">.rs</span>
        </span>
    </a>
</template>
