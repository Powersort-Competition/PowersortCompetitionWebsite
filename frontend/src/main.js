import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import vue3GoogleLogin from 'vue3-google-login'

const app = createApp(App)

app.use(vue3GoogleLogin,
    {
        clientId: 'PLACEHOLDER'
    })

app.mount('#app')
