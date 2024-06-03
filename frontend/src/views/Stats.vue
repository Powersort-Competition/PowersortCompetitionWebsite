<template>
  <div class="body">
    <h1>Heavyweight</h1>

    <suspense>
      <HeavyweightLeaderboard />
    </suspense>

    <h1>Mediumweight</h1>

    <suspense>
      <MediumweightLeaderboard />
    </suspense>

    <h1>Flyweight</h1>

    <suspense>
      <FlyweightLeaderboard />
    </suspense>
  </div>
</template>

<script setup>
// import Chart from 'chart.js';

import HeavyweightLeaderboard from "@/components/HeavyweightLeaderboard.vue";
import MediumweightLeaderboard from "@/components/MediumweightLeaderboard.vue";
import FlyweightLeaderboard from "@/components/FlyweightLeaderboard.vue";
</script>

<style scoped></style>
