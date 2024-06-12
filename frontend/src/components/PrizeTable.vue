<template>
  <div>
    <BTable striped hover :items="items"></BTable>
  </div>
</template>

<script setup>
import {BTable} from "bootstrap-vue-next";

const items = [
      { position: '1st', prize: 'Some prize here...' },
      { position: '2nd', prize: 'Some prize here...' },
      { position: '3rd', prize: 'Some prize here...' },
]
</script>