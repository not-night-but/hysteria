import { createApp } from 'vue'
import { createPinia } from 'pinia'
import FloatingVue from 'floating-vue'
import HysteriaStorePlugin from './plugins/HysteriaStorePlugin'

import App from './App.vue'
// import router from './router'

import './assets/styles/app.scss'
import 'material-icons/iconfont/material-icons.css'
import 'floating-vue/dist/style.css'

const app = createApp(App);

app.use(createPinia());
// app.use(router)
app.use(HysteriaStorePlugin);
app.use(FloatingVue, { defaultHtml: false });

app.mount('#app');
