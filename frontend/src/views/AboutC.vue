<template>
  <div class="container mt-4 main">
    <h1>
      <font-awesome-icon icon="code-merge"/>
      Track C: Commit (to) the future of sorting!
    </h1>
    <p>
      In Track C, you will contribute towards the <strong>future</strong> of sorting by
      integrating Powersort into a software framework that does not currently use it.
    </p>
    <p>
      A valid submission for Track C consists of a link to the public pull request on GitHub
      or another public code hosting platform where you have implemented Powersort.
      You will have to provide a brief description of the software framework and the
      changes you made to integrate Powersort, as well as what sorting algorithms were
      used before.
      The submission also has to prove that you have contributed to the pull request yourself
      in a substantial way.
    </p>
    <h2>Prizes</h2>
    For Track C, prizes will be individually selected among all submission by the
    academic organizers. The total prize pool for Track C is £1000.

  </div>
</template>

<script setup>
import {BCard, BCardGroup, BCardText, BLink} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
</script>
