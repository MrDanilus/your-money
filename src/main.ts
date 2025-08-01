import { createApp } from "vue";
import { useDark } from '@vueuse/core';
import App from "./App.vue";

const app = createApp(App);
app.mixin({
  setup() {
    useDark({
      selector: 'html',
      attribute: 'class',
      storageKey: 'vue-theme',
      storage: localStorage,
      valueDark: 'dark',
      valueLight: 'light',
    });
  },
});
app.mount('#app')