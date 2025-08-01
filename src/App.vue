<script setup lang="ts">
import { onBeforeMount, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import { Operation } from './scripts/operations';
import OperationsTable from './components/OperationsTable.vue';
import OperationsGraph from './components/OperationsGraph.vue';
import FundsTotal from './components/FundsTotal.vue';
import ThemeToggler from './components/ThemeToggler.vue';

defineEmits(['update']);

const error = ref<string>("");
const operations = ref<Operation[]>([]);

async function fetchOperations() {
    try {
        const result: Operation[] = await invoke("get_all_operations");
        operations.value = result;
    } catch (err: any) {
        error.value = err;
    }
};
onBeforeMount(fetchOperations);

async function update() {
    await fetchOperations();
}
</script>

<template>
    <div v-if="error" class="error">
        {{ error }}
    </div>
    <div v-else class="main">
        <OperationsGraph style="grid-column: 1;" :operations="operations"/>
        <FundsTotal :operations="operations" @update="update"/>
        <OperationsTable style="grid-column: 1;" :operations="operations"/>
        <ThemeToggler/>
    </div>
</template>

<style>
@import "/src/styles/app.css";

.main{
    --margin: 16px;

    width: auto;
    height: 100vh;
    margin: var(--margin);

    display: grid;
    gap: 12px;
    grid-template-columns: 3fr 1fr;
    grid-template-rows: 1.5fr 1fr;
}

.block{
    padding: 4px;
    border-radius: 16px;
    border: 1px solid rgb(228, 228, 228);
    transition: border 0.25s ease-in-out;
}
html.dark .block{
    border-color: rgb(49, 49, 49);
}
.block:hover{
    border-color: rgb(194, 194, 194);
}
html.dark .block:hover{
    border-color: rgb(68, 68, 68);
}

.not-found{
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center; /* Vertical */
    justify-content: center; /* Horizontal */
    color: gray;
    user-select: none;
}
.not-found p{
    text-align: center;
}
</style>