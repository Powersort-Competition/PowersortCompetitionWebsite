<template>
  <div class = "login">
    In order to upload a submission, you need to log in using a Google account.

    <br><br>
    <GoogleLogin :callback="callback"/>
  </div>
</template>

<script setup>
import { GoogleLogin } from "vue3-google-login";
import { decodeCredential } from "vue3-google-login";

const callback = (response) => {
  console.log("Google login raw response: ", response)

  const decoded_res = decodeCredential(response.credential)

  console.log("Google login decoded response: ", decoded_res)
}
</script>
