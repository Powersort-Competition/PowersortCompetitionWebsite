<template>
  <div class="home">
    <h1>Powersort Competition</h1>

    <div class="homeicon">
      <IIcBaselineFilterList/>
    </div>
    <br/>
    <Suspense>
      <Leaderboard/>
    </Suspense>
  </div>
</template>

<script setup>
import Leaderboard from "@/components/Leaderboard.vue";
</script>
