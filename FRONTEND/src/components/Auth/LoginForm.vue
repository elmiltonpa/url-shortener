<script setup lang="ts">
import { ref } from "vue";
import { Mail, Lock, ArrowRight } from "lucide-vue-next";
import Button from "../ui/Button/Button.vue";
import Input from "../ui/Input.vue";
import Logo from "../ui/Logo.vue";

const email = ref("");
const password = ref("");

const handleSubmit = () => {
    console.log("Logging in:", {
        email: email.value,
        password: password.value,
    });
};

const loginWithGoogle = () => {
    console.log("Google login");
};
</script>

<template>
    <div
        class="w-full max-w-md space-y-8 animate-in fade-in slide-in-from-bottom-5 duration-500"
    >
        <div class="flex flex-col items-center space-y-6 text-center">
            <Logo size="lg" />

            <div class="space-y-2">
                <h1 class="text-3xl font-bold tracking-tight">Welcome back</h1>
                <p class="text-muted-foreground">
                    Enter your credentials to access your account
                </p>
            </div>
        </div>

        <form @submit.prevent="handleSubmit" class="space-y-4">
            <div class="space-y-4">
                <Input
                    name="email"
                    placeholder="Email address"
                    type="email"
                    v-model="email"
                    required
                >
                    <template #icon>
                        <Mail class="h-4 w-4" />
                    </template>
                </Input>

                <Input
                    name="password"
                    placeholder="Password"
                    type="password"
                    v-model="password"
                    required
                >
                    <template #icon>
                        <Lock class="h-4 w-4" />
                    </template>
                </Input>
            </div>

            <Button
                type="submit"
                class="w-full py-6 text-base shadow-lg shadow-primary/10 transition-all hover:shadow-primary/20 active:scale-[0.98]"
            >
                Log in
                <ArrowRight class="ml-2 h-4 w-4" />
            </Button>
        </form>

        <div class="relative">
            <div class="absolute inset-0 flex items-center">
                <span class="w-full border-t border-border" />
            </div>
            <div class="relative flex justify-center text-xs uppercase">
                <span class="bg-background px-2 text-muted-foreground">
                    Or continue with
                </span>
            </div>
        </div>

        <Button
            variant="outline"
            type="button"
            class="w-full py-6 text-base border-border bg-card/50 hover:bg-accent transition-all active:scale-[0.98]"
            @click="loginWithGoogle"
        >
            <svg
                class="mr-2 h-4 w-4"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 488 512"
            >
                <path
                    fill="currentColor"
                    d="M488 261.8C488 403.3 391.1 504 248 504 110.8 504 0 393.2 0 256S110.8 8 248 8c66.8 0 123 24.5 166.3 64.9l-67.5 64.9C258.5 52.6 94.3 116.6 94.3 256c0 86.5 69.1 156.6 153.7 156.6 98.2 0 135-70.4 140.8-106.9H248v-85.3h236.1c2.3 12.7 3.9 24.9 3.9 41.4z"
                ></path>
            </svg>
            Sign in with Google
        </Button>

        <p class="text-center text-sm text-muted-foreground">
            Don't have an account?
            <a
                href="/register"
                class="font-medium text-primary underline-offset-4 hover:underline"
            >
                Sign up
            </a>
        </p>
    </div>
</template>
