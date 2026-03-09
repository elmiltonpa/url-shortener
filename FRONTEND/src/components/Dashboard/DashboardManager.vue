<script setup lang="ts">
import { useDashboard } from "../../composables/useDashboard";
import DashboardHeader from "./DashboardHeader.vue";
import UrlStats from "./UrlStats.vue";
import DashboardFilters from "./DashboardFilters.vue";
import UrlTable from "./UrlTable.vue";

const {
    loading,
    searchQuery,
    filteredUrls,
    stats,
    fetchUrls,
    errorMessage,
} = useDashboard();
</script>

<template>
    <div class="py-8">
        <!-- Error State -->
        <div
            v-if="errorMessage"
            class="w-full max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 mb-8 animate-in fade-in slide-in-from-top-4 duration-300"
        >
            <div
                class="bg-red-500/10 border border-red-500/20 p-5 rounded-2xl backdrop-blur-md flex flex-col sm:flex-row items-center justify-between gap-6"
            >
                <div class="flex items-center gap-4 text-center sm:text-left">
                    <div
                        class="h-12 w-12 rounded-2xl bg-red-500/20 flex items-center justify-center text-red-500 shrink-0 shadow-lg shadow-red-500/10"
                    >
                        <span class="text-xl font-bold">!</span>
                    </div>
                    <div>
                        <p class="text-base font-bold text-red-500">
                            Service Unavailable
                        </p>
                        <p class="text-sm text-red-500/70 font-medium">
                            {{ errorMessage }}
                        </p>
                    </div>
                </div>
                <button
                    @click="fetchUrls"
                    class="w-full sm:w-auto px-6 py-2.5 rounded-xl bg-red-500 text-white text-sm font-bold hover:bg-red-600 active:scale-95 transition-all shadow-lg shadow-red-500/20 cursor-pointer"
                >
                    Retry Connection
                </button>
            </div>
        </div>

        <DashboardHeader :loading="loading" @refresh="fetchUrls" />

        <UrlStats :stats="stats" :loading="loading" />

        <DashboardFilters
            v-model="searchQuery"
            :totalCount="filteredUrls.length"
            :loading="loading"
        />

        <UrlTable :urls="filteredUrls" :loading="loading" />
    </div>
</template>
