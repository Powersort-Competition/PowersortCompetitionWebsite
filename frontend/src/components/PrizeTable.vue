<template>
  <div>
    <BTable striped hover :items="items"></BTable>
  </div>
</template>

<script setup>
import {BTable} from "bootstrap-vue-next";

const items = [
  {position: '1st', prize: '£2⁷ = £128 amazon voucher'},
  {position: '2nd', prize: '£2⁶ = £64 amazon voucher'},
  {position: '3rd', prize: '£2⁵ = £32 amazon voucher'},
]
</script>
