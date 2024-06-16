<template>
  <BAlert :model-value="true" variant="danger">
    <p>
      We are currently experiencing issues with the server. As a result, submissions are temporarily disabled.
      Please try again later.
    </p>
  </BAlert>
</template>

<script setup>
import {BAlert} from "bootstrap-vue-next";
</script>
