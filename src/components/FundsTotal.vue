<script setup lang="ts">
import { onUpdated, ref } from 'vue';
import { Operation } from '../scripts/operations';
import Expense from './FundsModals/Expense.vue';
import Income from './FundsModals/Income.vue';

defineEmits(['update']);
const props = defineProps<{
    operations: Operation[];
}>();

const amount = ref<string>("");
const isToggled = ref<boolean>(false);

const modal = ref<string>("");

onUpdated(() => {
    let money = props.operations.reduce(
        (partialSum, operation) => {
            if (operation.kind === "Income"){
                return partialSum + operation.amount
            } else{
                return partialSum - operation.amount
            }
        }, 0
    );

    let moneyABS = Math.abs(money);
    switch (true){
        case (moneyABS < 1000):
            amount.value = money.toString();
            break;
        case (moneyABS < 1000000):
            amount.value = (money/1000).toFixed(1).toString() + "k";
            break;
        case (moneyABS < 1000000000):
            amount.value = (money/1000/1000).toFixed(1).toString() + "M";
            break;
        case (moneyABS < 1000000000000):
            amount.value = (money/1000/1000/1000).toFixed(1).toString() + "B";
            break;
        case (moneyABS < 1000000000000000):
            amount.value = (money/1000/1000/1000/1000).toFixed(1).toString() + "T";
            break;
        default:
            amount.value = (money/1000/1000/1000/1000/1000).toFixed(1).toString() + "Q";
            break;
    }
})
</script>

<template>
    <div class="block" style="position: relative">
        <div class="block-inner" :class="{ 'rotate': isToggled }" @click="isToggled = !isToggled">
            <div class="funds">
                <div class="amount">
                    <span v-if="operations.length === 0">-.-</span>
                    <span v-else>{{ amount }} ₽</span>  
                </div>
                <p>↩️</p>
            </div>
            <div class="actions">
                <button @click.stop="modal = 'income'" style="--bg-color: #50C878;">Доход</button>
                <button @click.stop="modal = 'expense'" style="--bg-color: #D22B2B;">Расход</button>
                <p>↩️</p>
            </div>
        </div>
    </div>
    <div class="modals" @click="modal = ''"
        v-if="modal === 'income' || modal === 'expense'">
        <Income v-if="modal === 'income'" @close="modal = ''" @update="$emit('update')"/>
        <Expense v-if="modal === 'expense'" @close="modal = ''" @update="$emit('update')"/>
    </div>
</template>

<style scoped>
.block{
    cursor: pointer;
    perspective: 1000px;
}
.block-inner{
    position: relative;
    width: 100%;
    height: 100%;
    transition: transform 0.4s;
    transform-style: preserve-3d;
}
.block-inner.rotate{
  transform: rotateY(180deg);
}

.funds,
.actions {
  position: absolute;
  width: 100%;
  height: 100%;
  backface-visibility: hidden;
  display: flex;
  border-radius: 0.5rem;
}

.funds{
    display: grid;
    gap: 8px;
    grid-template-rows: 1fr 0.5fr;

    font-size: 48px;
    align-items: center;
    justify-content: center;
}
.funds.amount{
    margin: 0;
    text-align: center;
    font-size: 24px;
    transition: all 0.25s;
}
.funds p{
    user-select: none;
    opacity: 0;
    text-align: center;
    font-size: 24px;
    transition: all 0.25s;
}
.funds:hover p{
    opacity: 100;
}

.actions{
    user-select: none;
    transform: rotateY(180deg);
    
    display: grid;
    gap: 8px;
    grid-template-rows: 1fr 1fr 0.5fr;
}
.actions p{
    opacity: 0;
    text-align: center;
    font-size: 24px;
    transition: all 0.25s;
}
.actions:hover p{
    opacity: 100;
}

.actions button{
    background-color: var(--bg-color);
    padding: 12px;
    border-radius: 12px;
    color: white;
    font-weight: bold;
    font-size: 14px;
    cursor: pointer;
}

.modals{
    z-index: 10;
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background: rgba(0, 0, 0, 0.5);
}
</style>