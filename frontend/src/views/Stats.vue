<template>
  <div class="container mt-4">
    <h1>Leaderboards</h1>
    <BFormRadioGroup
      v-model="selectedMetricType"
      :options="metricTypes"
      class="mb-10 text-right"
      value-field="item"
      text-field="name"
    />
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

import {BFormRadioGroup} from "bootstrap-vue-next";
import {ref} from "vue";

const selectedMetricType = ref('perc_diff');
const metricTypes = [
  {item: 'perc_diff', name: 'Merge Cost Difference (%)'},
  {item: 'comp_diff', name: 'Comparison Count Difference'}
]
</script>

<style scoped></style>
