<script setup lang="ts">
import { reactive } from "vue";
import { User, Mail, Lock, ArrowRight, Loader2 } from "lucide-vue-next";
import { actions, isActionError, isInputError } from "astro:actions";
import { useFormSubmit } from "../../composables/useFormSubmit";
import Button from "../ui/Button/Button.vue";
import Input from "../ui/Input.vue";
import Logo from "../ui/Logo.vue";

const formData = reactive({
    username: "",
    email: "",
    password: "",
    confirmPassword: "",
});

const { isLoading, errorMessage, handleSubmit } = useFormSubmit();

const onSubmit = () =>
    handleSubmit(async () => {
        if (formData.password !== formData.confirmPassword) {
            errorMessage.value = "Passwords do not match";
            return;
        }

        const { data, error } = await actions.register({
            username: formData.username,
            email: formData.email,
            password: formData.password,
        });

        if (error) {
            if (isInputError(error)) {
                const errors = Object.values(error.fields).flat();
                errorMessage.value = errors[0] || "Invalid input";
            } else if (isActionError(error)) {
                errorMessage.value = error.message;
            } else {
                errorMessage.value = "An unexpected error occurred";
            }
        } else if (data) {
            window.location.href = "/";
        }
    });

const loginWithGoogle = () => {
    window.location.href = "/api/auth/google";
};
</script>

<template>
    <div
        class="w-full max-w-md space-y-8 animate-in fade-in slide-in-from-bottom-5 duration-500"
    >
        <div class="flex flex-col items-center space-y-6 text-center">
            <Logo size="lg" />

            <div class="space-y-2">
                <h1 class="text-3xl font-bold tracking-tight">
                    Create an account
                </h1>
                <p class="text-muted-foreground">
                    Enter your details below to create your account
                </p>
            </div>
        </div>

        <form @submit.prevent="onSubmit" class="space-y-4">
            <div
                v-if="errorMessage"
                class="p-3 text-sm font-medium text-destructive bg-destructive/10 rounded-md border border-destructive/20"
            >
                {{ errorMessage }}
            </div>

            <div class="space-y-4">
                <Input
                    name="username"
                    placeholder="Username"
                    type="text"
                    v-model="formData.username"
                    required
                    :disabled="isLoading"
                >
                    <template #icon>
                        <User class="h-4 w-4" />
                    </template>
                </Input>

                <Input
                    name="email"
                    placeholder="Email address"
                    type="email"
                    v-model="formData.email"
                    required
                    :disabled="isLoading"
                >
                    <template #icon>
                        <Mail class="h-4 w-4" />
                    </template>
                </Input>

                <Input
                    name="password"
                    placeholder="Password"
                    type="password"
                    v-model="formData.password"
                    required
                    :disabled="isLoading"
                >
                    <template #icon>
                        <Lock class="h-4 w-4" />
                    </template>
                </Input>

                <Input
                    name="confirmPassword"
                    placeholder="Repeat Password"
                    type="password"
                    v-model="formData.confirmPassword"
                    required
                    :disabled="isLoading"
                >
                    <template #icon>
                        <Lock class="h-4 w-4" />
                    </template>
                </Input>
            </div>

            <Button
                type="submit"
                :disabled="isLoading"
                class="w-full py-6 text-base shadow-lg shadow-primary/10 transition-all hover:shadow-primary/20 active:scale-[0.98]"
            >
                <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
                <template v-else>
                    Create account
                    <ArrowRight class="ml-2 h-4 w-4" />
                </template>
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
            :disabled="isLoading"
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
                />
            </svg>
            Sign in with Google
        </Button>

        <p class="text-center text-sm text-muted-foreground">
            Already have an account?
            <a
                href="/login"
                class="font-medium text-primary underline-offset-4 hover:underline"
            >
                Log in
            </a>
        </p>
    </div>
</template>
