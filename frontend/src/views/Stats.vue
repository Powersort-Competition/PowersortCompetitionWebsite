<template>
  <div class="container mt-4">
    <h1>Leaderboards</h1>
    <p class="mb-10 text-right">
      <strong>Cost Measure:</strong> &nbsp;
    <BFormRadioGroup
      v-model="selectedMetricType"
      :options="metricTypes"
      value-field="item"
      text-field="name"
      buttons
      button-variant="outline-primary"
    />
    </p>

    <h2>Heavyweight</h2>

    <suspense>
      <HeavyweightLeaderboard :selectedMetric="selectedMetricType"/>
    </suspense>

    <br>
    <h2>Mediumweight</h2>

    <suspense>
      <MediumweightLeaderboard :selectedMetric="selectedMetricType"/>
    </suspense>

    <br>
    <h2>Flyweight</h2>

    <suspense>
      <FlyweightLeaderboard :selectedMetric="selectedMetricType"/>
    </suspense>
  </div>
</template>

<script setup>
import HeavyweightLeaderboard from "@/components/HeavyweightLeaderboard.vue";
import MediumweightLeaderboard from "@/components/MediumweightLeaderboard.vue";
import FlyweightLeaderboard from "@/components/FlyweightLeaderboard.vue";

import {BFormRadioGroup} from "bootstrap-vue-next";
import {ref} from "vue";
//import TrackACompositionDoughnut from "@/components/TrackACompositionDoughnut.vue";

const selectedMetricType = ref('perc_diff');
const metricTypes = [
  {item: 'perc_diff', name: 'Merge Cost Difference (%)'},
  {item: 'comp_diff', name: 'Comparison Count Difference'}
]
</script>

<style scoped></style>
