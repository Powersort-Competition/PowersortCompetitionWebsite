<template>
  <div class="useful">
    <h1>Background</h1>

    <h1>Competition Details</h1>
    The competition is split into three discrete classes - <i>flyweight</i>, <i>mediumweight</i> and <i>heavyweight</i>.
    Each submission is allocated a specific class based on its number of elements. As such, we aim to capture anything
    from edge cases in small inputs that might cause Timsort to perform greater than Powersort, to larger and more
    complex inputs.

    <br>
    <br>
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

    <br>
    <br>
    There are prizes to be won for each submission class. You are welcome to submit as many times as you like, and for
    any of the class!

    <br>
    <br>
    <br>
    <h2>Prizes</h2>

    <PrizeTable/>
  </div>
</template>

<script setup>
import {BCard, BCardGroup, BCardText} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
</script>
