<template>
  <div class="home">
    <h1>Powersort Competition</h1>

    <div class="homeicon">
      <IIcBaselineFilterList/>
    </div>
    <p>
      Please read the <BLink to="about">about</BLink>
      page to learn about the different competition tracks; what they consist of,
      and the prizes to be won!
    </p>
    <br/>
    <Suspense>
      <Leaderboard/>
    </Suspense>
  </div>
</template>

<script setup>
import Leaderboard from "@/components/Leaderboard.vue";
import {BLink} from "bootstrap-vue-next";
</script>
