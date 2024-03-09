import { createApp } from 'vue'
import './style.css'

import App from './App.vue'
import Home from './views/Home.vue'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap'
import vue3GoogleLogin from 'vue3-google-login'
import router from './router'

const app = createApp(App).use(router)

app.use(vue3GoogleLogin,
    {
        clientId: 'PLACEHOLDER'
    })

app.mount('#app')
