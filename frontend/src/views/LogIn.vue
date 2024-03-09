<template>
  <div class = "login">
    Hello!

    <GoogleLogin :callback="callback"/>
  </div>
</template>

<script setup>
import {GoogleLogin} from "vue3-google-login";

const callback = (response) => {
  // This callback will be triggered when the user selects or login to
  // his Google account from the popup
  console.log("Handle the response", response)
}
</script>