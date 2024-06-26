<template>
  <div class="container mt-4 main">
    <div v-if="logInSuccess === true">
      <BAlert variant="success">You have logged in successfully. You will now be redirected to the home page.</BAlert>
    </div>
    <h1>Login</h1>
    <p v-if="$cookies.get('pscomp_oauth') == null">
      In order to upload a submission, you need to log in using a Google
      account.

      Your name will be shown on the leaderboard, and you will need the submission
      receipt emails to claim any prizes.

      <br/><br/>
      <GoogleLogin :callback="callback"/>
    </p>
    <p v-else>You are already logged in as {{ email }}.</p>
  </div>
</template>

<script setup>
import {ref} from "vue";

import {decodeCredential, GoogleLogin} from "vue3-google-login";
import router from "@/router/index.js";

import {BAlert} from "bootstrap-vue-next";

let logInSuccess = ref(false);

let email;
if ($cookies.get("pscomp_oauth") != null) {
  email = $cookies.get("pscomp_oauth").email;
}

const callback = (response) => {
  console.log("Google login raw response: ", response);

  const decoded_res = decodeCredential(response.credential);

  console.log("Google login decoded response: ", decoded_res);

  // Probe API and store data if new user.
  const servResponse = ref(null);
  const requestOptions = {
    method: "POST",
    headers: {
      "content-type": "application/json",
      "Access-Control-Allow-Origin": "*",
    },
    body: JSON.stringify({
      id: 0,
      first_name: decoded_res["given_name"],
      last_name: decoded_res["family_name"],
      email: decoded_res["email"],
    }),
  };

  fetch("https://psortcomp.shayandoust.me/logged_in", requestOptions)
      .then((response) => response.json())
      .then((data) => (servResponse.value.status = data));

  console.log("Server replied with: ", response);

  $cookies.set("pscomp_oauth", JSON.stringify(decoded_res));

  // Once logged in, route to home page after 3 seconds.
  logInSuccess.value = true;
  setTimeout(() => {
    router.push({name: "home"});
  }, 3000);
};
</script>
