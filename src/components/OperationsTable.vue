<script setup lang="ts">
import { Operation } from '../scripts/operations';
const props = defineProps<{
    operations: Operation[];
}>();
</script>

<template>
    <div class="block">
        <div v-if="operations.length === 0" class="not-found">
            <p>Операции не найдены</p>
        </div>
        <div class="table" v-else>
            <div v-for="operation in props.operations">
                {{ operation }}
            </div>
        </div>
    </div>
</template>

<style scoped>
.not-found{
    opacity: 0;
    transition: all 0.25s ease-in-out;
}
.not-found:hover{
    opacity: 100;
}

.table{
    overflow-y: scroll;
}
</style>