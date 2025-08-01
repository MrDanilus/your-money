<script setup lang="ts">
import Chart from 'chart.js/auto';
import { Operation } from '../scripts/operations';
import { onUpdated, ref } from 'vue';

const props = defineProps<{
    operations: Operation[];
}>();
const chart = ref<Chart>();

onUpdated(() => {
    if (chart.value){ chart.value.destroy(); };

    const canvas = document.getElementById('acquisitions') as HTMLCanvasElement;
    let incomes: Operation[] = [];
    let expenses: Operation[] = [];
    props.operations.map(operation => {
        if (operation.kind === "Income"){
            incomes.push(operation);
        } else{
            expenses.push(operation);
        }
    });

    const labels = [...new Set(
        props.operations.map(row => 
            new Date(parseInt(row.timestamp) * 1000).toLocaleDateString('en-CA')))
    ];
    chart.value = new Chart(
        canvas,
        {
            type: 'line',
            data: {
                labels,
                datasets: [{
                    label: 'Доход',
                    fill: true,
                    data: incomes.map(row => row.amount),
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.1
                },
                {
                    label: 'Расход',
                    fill: true,
                    data: expenses.map(row => row.amount),
                    borderColor: 'rgb(192, 75, 75)',
                    tension: 0.1
                }]
            }
        }
    );
})
</script>

<template>
    <div class="block">
        <div v-if="operations.length === 0" class="not-found">
            <p>
                Операции не найдены
                <br>
                Вы можете добавить их справа ➡️
            </p>
        </div>
        <div v-else class="graph">
            <canvas id="acquisitions"></canvas>
        </div>
    </div>
</template>

<style scoped>
.graph {
    width: 100%;
    height: 100%;
}
</style>