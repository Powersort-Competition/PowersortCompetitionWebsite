import "./style.scss";
import {createApp} from "vue";

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


/* import the fontawesome core */
import { library, config } from '@fortawesome/fontawesome-svg-core'

/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

/* import VueKatex component */
import VueKatex from '@hsorby/vue3-katex'
import 'katex/dist/katex.min.css'

// import icon style(s)
import { fas } from "@fortawesome/free-solid-svg-icons";

/* add icons to the library */
library.add(fas)


const app = createApp(App).use(router);
const vuetify = createVuetify({
    theme: {
        defaultTheme: 'dark',
    }
});

axios.defaults.baseURL = import.meta.env.VITE_BACKEND_URL;

app.use(createBootstrap());
app.use(VueKatex)
app.use(VueCookies, {expires: "6h"});
app.use(vue3GoogleLogin, {
    clientId:
        "769935082895-a3dirndnnbcc6cdlig4at7650p73n3cl.apps.googleusercontent.com",
});
app.use(vuetify);
app.component('font-awesome-icon', FontAwesomeIcon)

app.mount("#app");
