<template>
  <div class="stats">
    <h1>Leaderboards</h1>
    <h2>Heavyweight</h2>

    <suspense>
      <HeavyweightLeaderboard/>
    </suspense>

    <br>
    <h2>Mediumweight</h2>

    <suspense>
      <MediumweightLeaderboard/>
    </suspense>

    <br>
    <h2>Flyweight</h2>

    <suspense>
      <FlyweightLeaderboard/>
    </suspense>
  </div>

  <!--  <ScatterGraph />-->
</template>

<script setup>
import HeavyweightLeaderboard from "@/components/HeavyweightLeaderboard.vue";
import MediumweightLeaderboard from "@/components/MediumweightLeaderboard.vue";
import FlyweightLeaderboard from "@/components/FlyweightLeaderboard.vue";
</script>

<style scoped></style>
