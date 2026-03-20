<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import {
    User,
    LogOut,
    ChevronDown,
    LayoutDashboard,
    Home,
} from "lucide-vue-next";
import { actions } from "astro:actions";

const props = defineProps<{
    user: {
        username: string;
        email: string;
    };
}>();

const isOpen = ref(false);

const toggleMenu = () => {
    isOpen.value = !isOpen.value;
};

const handleClickOutside = (e: MouseEvent) => {
    const target = e.target as HTMLElement;
    if (!target.closest(".user-menu-container")) {
        isOpen.value = false;
    }
};

onMounted(() => {
    window.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
    window.removeEventListener("click", handleClickOutside);
});

const handleLogout = async () => {
    try {
        const result = await actions.logout();
        if (result.error) {
            console.error("Logout error:", result.error);
        }
    } catch (err) {
        console.error("Unexpected logout error:", err);
    }
    // Always redirect - use href assignment for a full page reload
    // This ensures cookies are re-read from scratch
    window.location.href = "/";
};
</script>

<template>
    <div class="relative user-menu-container">
        <button
            type="button"
            @click="toggleMenu"
            class="flex items-center gap-2 p-1.5 pl-3 rounded-full border border-border/50 bg-background md:bg-card/30 backdrop-blur-md hover:bg-card/50 transition-all active:scale-[0.98] group"
        >
            <span
                class="text-sm font-medium text-foreground/90 group-hover:text-foreground"
            >
                {{ user.username }}
            </span>
            <div
                class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center border border-primary/20 text-primary"
            >
                <User class="w-4 h-4" />
            </div>
            <ChevronDown
                class="w-4 h-4 text-muted-foreground transition-transform duration-200"
                :class="{ 'rotate-180': isOpen }"
            />
        </button>

        <div
            v-if="isOpen"
            class="absolute right-0 mt-2 w-56 p-1.5 rounded-xl border border-border/50 bg-background backdrop-blur-xl shadow-2xl animate-in fade-in zoom-in-95 duration-200 z-50"
        >
            <div class="px-3 py-2 border-b border-border/30 mb-1">
                <p
                    class="text-xs font-medium text-muted-foreground uppercase tracking-wider"
                >
                    Account
                </p>
                <p class="text-sm font-semibold truncate">{{ user.email }}</p>
            </div>

            <a
                href="/"
                class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-foreground/80 hover:bg-accent hover:text-foreground transition-colors"
            >
                <Home class="w-4 h-4" />
                Home
            </a>

            <a
                href="/dashboard"
                class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-foreground/80 hover:bg-accent hover:text-foreground transition-colors"
            >
                <LayoutDashboard class="w-4 h-4" />
                Dashboard
            </a>

            <div class="h-px bg-border/30 my-1 mx-1"></div>

            <button
                type="button"
                @click.prevent="handleLogout"
                class="w-full flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-destructive hover:bg-destructive/10 transition-colors"
            >
                <LogOut class="w-4 h-4" />
                Log out
            </button>
        </div>
    </div>
</template>
