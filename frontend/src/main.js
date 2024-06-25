import {createApp} from "vue";
import "./style.css";

import App from "./App.vue";

import axios from "axios";
import vue3GoogleLogin from "vue3-google-login";
import VueCookies from "vue-cookies";
import router from "./router";

// Bootstrap imports.
import {createBootstrap} from 'bootstrap-vue-next'
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue-next/dist/bootstrap-vue-next.css'

// Vuetify imports.
import 'vuetify/styles'
import {createVuetify} from "vuetify";

const app = createApp(App).use(router);
const vuetify = createVuetify();

axios.defaults.baseURL = import.meta.env.VITE_BACKEND_URL;

app.use(createBootstrap());
app.use(VueCookies, {expires: "6h"});
app.use(vue3GoogleLogin, {
    clientId:
        "769935082895-a3dirndnnbcc6cdlig4at7650p73n3cl.apps.googleusercontent.com",
});
app.use(vuetify);

app.mount("#app");
