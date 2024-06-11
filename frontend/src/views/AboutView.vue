<template>
  <div class="useful">
    <h1>Background</h1>

    <h1>Competition Details</h1>
    <BCardGroup deck>
    <BCard bg-variant="light" header="Flyweight" class="text-center">
      <BCardText>Submissions where the number of elements is less than <code>10,000</code>.</BCardText>
    </BCard>
    <BCard bg-variant="light" header="Mediumweight" class="text-center">
      <BCardText>Submissions where the number of elements is at least <code>10,000</code>
        and at most <code>1,000,000</code>.</BCardText>
    </BCard>
    <BCard bg-variant="light" header="Heavyweight" class="text-center">
      <BCardText>Submissions where the number of elements is equal to or greater than
      <code>1,000,000</code>.</BCardText>
    </BCard>

    </BCardGroup>
  </div>
</template>

<script setup>
import {BCard, BCardGroup, BCardText} from "bootstrap-vue-next";
</script>
