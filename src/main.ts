import { createApp } from 'vue'
import { createPinia } from 'pinia'
import HysteriaStorePlugin from './plugins/HysteriaStorePlugin'

import App from './App.vue'
// import router from './router'

import './assets/styles/app.scss'

const app = createApp(App)

app.use(createPinia())
// app.use(router)
app.use(HysteriaStorePlugin)

app.mount('#app')
