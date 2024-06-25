<template>

  <h1>Powersort Competition</h1>

  <div class="homeicon">
    <IIcBaselineFilterList/>
  </div>
  <p>
    Help us as citizen scientist to improve Python's sorting algorithm!
    Win prizes by solving puzzles and help the climate! Every bit counts.
  </p>
  <p>
    Please read the
    <BLink to="about">about</BLink>
    page to learn about the different competition tracks; what they consist of,
    and the prizes to be won!
  </p>
  <br/>
  <Suspense>
    <Leaderboard/>
  </Suspense>

</template>

<script setup>
import Leaderboard from "@/components/Leaderboard.vue";
import {BLink} from "bootstrap-vue-next";
</script>
