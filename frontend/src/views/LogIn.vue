<template>
  <div class = "login">
    <p v-if="$cookies.get('pscomp_oauth') == null">
    In order to upload a submission, you need to log in using a Google account.

    <br><br>
    <GoogleLogin :callback="callback"/>
    </p>
    <p v-else>
      You are already logged in as {{ this.email }}.
    </p>
  </div>
</template>

<script setup>
import { ref } from 'vue';

import { GoogleLogin } from 'vue3-google-login';
import { decodeCredential } from 'vue3-google-login';

const callback = (response) => {
  console.log('Google login raw response: ', response)

  const decoded_res = decodeCredential(response.credential)
  this.email = decoded_res["email"]

  console.log('Google login decoded response: ', decoded_res)

  // Probe API and store data if new user.
  const servResponse = ref(null);
  const requestOptions = {
    method: 'POST',
    headers: { 'content-type': 'application/json',
               'Access-Control-Allow-Origin': '*'},
    body: JSON.stringify(
        { id: 0,
                first_name: decoded_res["given_name"],
                last_name: decoded_res["family_name"],
                email: decoded_res["email"] })
  }

  fetch('https://psortcomp.shayandoust.me/logged_in', requestOptions)
      .then(response => response.json())
      .then(data => servResponse.status = data);

  console.log("Server replied with: ", response);

  $cookies.set('pscomp_oauth', JSON.stringify(decoded_res))
}
</script>
