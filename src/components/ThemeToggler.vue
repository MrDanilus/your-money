<script setup lang="ts">
import { useDark } from '@vueuse/core'

const isDark = useDark({
    disableTransition: false,
    selector: 'html',
    attribute: 'class',
    storageKey: 'vue-theme',
    storage: localStorage,
    valueDark: 'dark',
    valueLight: 'light',
});
function toggleTheme() {
    isDark.value = !isDark.value;
}
</script>

<template>
    <button class="theme-toggle" @click="toggleTheme">
        <span v-if="!isDark">☀️</span>
        <span v-else>🌙</span>
    </button>
</template>

<style scoped>
.theme-toggle{
    cursor: pointer;
    font-size: 36px;
}
</style>