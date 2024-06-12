<template>
  <div class="stats">
    <h1>Heavyweight</h1>

    <suspense>
      <HeavyweightLeaderboard/>
    </suspense>

    <br>
    <h1>Mediumweight</h1>

    <suspense>
      <MediumweightLeaderboard/>
    </suspense>

    <br>
    <h1>Flyweight</h1>

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
