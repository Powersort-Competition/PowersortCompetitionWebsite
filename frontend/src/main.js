import { createApp } from "vue";
import "./style.css";

import App from "./App.vue";
import { backendHealthCheck } from "@/misc.js";

import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap";

import vue3GoogleLogin from "vue3-google-login";
import VueCookies from "vue-cookies";
import router from "./router";

const app = createApp(App).use(router);

// Conditionally, the backend needs to be up and running for the webpages to work.
if ((await backendHealthCheck()) == "pong") {
  app.use(VueCookies, { expires: "6h" });
  app.use(vue3GoogleLogin, {
    clientId:
      "769935082895-a3dirndnnbcc6cdlig4at7650p73n3cl.apps.googleusercontent.com",
  });

  app.mount("#app");
}
