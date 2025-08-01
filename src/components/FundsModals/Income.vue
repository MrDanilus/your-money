<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const emit = defineEmits(['close', 'update']);

const date = ref<string>(new Date().toLocaleDateString('en-CA'));
const description = ref<string>("");
const amount = ref<number>(0);

async function send(){
    await invoke("new_operation", {
        description: description.value,
        amount: amount.value,
        kind: "income",
        timestamp: Math.floor(new Date(date.value).getTime() / 1000).toString()
    }).then(() => {
        emit('update');
        emit('close');
    }).catch(err => {
        console.log(err);
    });
}
</script>

<template>
    <div class="modal" @click.stop>
        <div class="modal-header">
            <p>Новый доход</p>
            <button @click="$emit('close')">❌</button>
        </div>
        <div class="modal-body">
            <form @submit.prevent="send">
                <div>
                    <label>Дата</label>
                    <input v-model="date" type="date" 
                        required>
                </div>
                <div>
                    <label>Описание</label>
                    <input v-model="description" type="text" 
                        minlength="4" size="36" required>
                </div>
                <div>
                    <label>Сумма, ₽</label>
                    <input v-model="amount" type="number" step="0.01" 
                        min="1" max="9223372036854775807" required>
                </div>
                <input type="submit" value="Создать">
            </form>
        </div>
    </div>
</template>

<style scoped>
@import '/src/styles/modal.css';
@import '/src/styles/form.css';
</style>