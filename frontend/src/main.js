import { createApp } from 'vue'
import './style.css'

import App from './App.vue'


import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap'

import vue3GoogleLogin from 'vue3-google-login'
import VueCookies from 'vue-cookies'
import router from './router'

const app = createApp(App).use(router)

app.use(VueCookies, { expires: '1h' })
app.use(vue3GoogleLogin,
    {
        clientId: '769935082895-a3dirndnnbcc6cdlig4at7650p73n3cl.apps.googleusercontent.com'
    })

app.mount('#app')
