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

<!--  <ScatterGraph />-->
</template>

<script setup>
import HeavyweightLeaderboard from "@/components/HeavyweightLeaderboard.vue";
import MediumweightLeaderboard from "@/components/MediumweightLeaderboard.vue";
import FlyweightLeaderboard from "@/components/FlyweightLeaderboard.vue";

import ScatterGraph from "@/components/ScatterGraph.vue";
</script>

<style scoped></style>
