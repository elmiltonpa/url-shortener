<script setup lang="ts">
import { List, Search } from "lucide-vue-next";

const props = defineProps<{
    modelValue: string;
    totalCount: number;
    loading: boolean;
}>();

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void;
}>();

const updateValue = (e: Event) => {
    const target = e.target as HTMLInputElement;
    emit('update:modelValue', target.value);
};
</script>

<template>
    <div class="w-full max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 mb-6">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-card/30 backdrop-blur-md p-4 rounded-2xl border border-border">
            <div class="flex items-center gap-2">
                <List class="h-5 w-5 text-rust" />
                <h2 class="text-xl font-bold text-foreground">My Links</h2>
                <span v-if="!loading" class="ml-2 px-2 py-0.5 rounded-md bg-muted text-muted-foreground text-xs font-semibold">
                    {{ totalCount }}
                </span>
            </div>

            <div class="relative w-full sm:w-80">
                <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <input 
                    :value="modelValue"
                    @input="updateValue"
                    type="text" 
                    placeholder="Search by URL or code..." 
                    class="w-full bg-background/50 border border-border rounded-xl pl-10 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-rust/20 focus:border-rust/50 transition-all"
                />
            </div>
        </div>
    </div>
</template>
