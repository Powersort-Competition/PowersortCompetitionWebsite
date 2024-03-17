import { createApp } from 'vue'
import './style.css'

import App from './App.vue'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap'
import vue3GoogleLogin from 'vue3-google-login'
import VueCookies from 'vue-cookies'
import router from './router'

const app = createApp(App).use(router)

app.use(VueCookies, { expires: '1d' })
app.use(vue3GoogleLogin,
    {
        clientId: '1006096400271-vaib2i6q6dskcfn4q09hv1bigr2im27o.apps.googleusercontent.com'
    })

app.mount('#app')
