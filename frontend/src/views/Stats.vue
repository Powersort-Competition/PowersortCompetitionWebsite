<template>
  <div class="container mt-4">
    <h1>Leaderboards Track A</h1>
    <p class="mb-10 text-right">
      <strong>Sort by (descending order):</strong> &nbsp;
    <BFormRadioGroup
      v-model="selectedMetricType"
      :options="metricTypes"
      value-field="item"
      text-field="name"
      buttons
      button-variant="outline-primary"
    />
    </p>

    <h2><font-awesome-icon icon="ranking-star"/> Heavyweight</h2>

    <suspense>
      <HeavyweightLeaderboard :selectedMetric="selectedMetricType"/>
    </suspense>

    <br>
    <h2><font-awesome-icon icon="ranking-star"/> Mediumweight</h2>

    <suspense>
      <MediumweightLeaderboard :selectedMetric="selectedMetricType"/>
    </suspense>

    <br>
    <h2><font-awesome-icon icon="ranking-star"/> Flyweight</h2>

    <suspense>
      <FlyweightLeaderboard :selectedMetric="selectedMetricType"/>
    </suspense>

    <hr/>

    <h1 style="margin-top:2ex;">🏆 Past Winners Track A 🎉</h1>
    <p>The recipients of prizes in Track A are listed below.<br/>  Congratulations and thanks for your contribution to science!</p>
    <h3>Round 1</h3>
    <ul>
      <li>Chris J.</li>
      <li>Daniel C.</li>
      <li>Thomas J.</li>
    </ul>
    <h3>Round 2</h3>
    <ul>
      <li>Thomas J.</li>
      <li>Vincent J.</li>
      <li>Ziad I.</li>
    </ul>
    <h3>Round 3</h3>
    <p><em>You?</em></p>
  </div>
</template>

<script setup>
import HeavyweightLeaderboard from "@/components/HeavyweightLeaderboard.vue";
import MediumweightLeaderboard from "@/components/MediumweightLeaderboard.vue";
import FlyweightLeaderboard from "@/components/FlyweightLeaderboard.vue";

import {BFormRadioGroup} from "bootstrap-vue-next";
import {ref} from "vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
//import TrackACompositionDoughnut from "@/components/TrackACompositionDoughnut.vue";

const selectedMetricType = ref('comp_diff');
const metricTypes = [
  {item: 'comp_diff', name: 'Comparison Count Difference (%)'},
  {item: 'merge_diff', name: 'Merge Cost Difference (%)'},
  {item: 'combined_metric', name: 'Combined Score'}
]
</script>

<style scoped></style>
