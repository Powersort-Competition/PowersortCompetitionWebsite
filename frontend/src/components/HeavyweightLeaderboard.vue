<template>
  <Suspense>
    <div class="wide-table" >
    <table class="table" id="heavyweightTBL">
      <caption>
        Top 5 heavyweight submissions
      </caption>
      <thead>
      <tr>
        <th scope="col">Submission ID</th>
        <th scope="col">Submitter</th>
        <th scope="col">Powersort Comparisons</th>
        <th scope="col">Timsort Comparisons</th>
        <th scope="col">Powersort Merge Cost</th>
        <th scope="col">Timsort Merge Cost</th>
        <th scope="col">Comparison Difference (%)</th>
        <th scope="col">Merge Cost Difference (%)</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="submission in leaderboardJson">
        <td>{{ submission.submission_id }}</td>
        <td>{{ submission.submitter }}</td>
        <td>{{ submission.powersort_comp }}</td>
        <td>{{ submission.timsort_comp }}</td>
        <td>{{ submission.powersort_merge_cost }}</td>
        <td>{{ submission.timsort_merge_cost }}</td>
        <td>{{ submission.comp_diff }}</td>
        <td>{{ submission.mcost_diff }}</td>
      </tr>
      </tbody>
    </table>
    </div>
  </Suspense>
</template>

<script setup>
import {defineProps, watch} from "vue";
import {sortTable} from "@/misc.js"

const props = defineProps({
  selectedMetric: String,
  default: "comp_diff"
});

watch(() => props.selectedMetric, (newType) => {
  if (newType === "merge_diff") {
    sortTable(document.getElementById("heavyweightTBL"), 7);
  } else {
    sortTable(document.getElementById("heavyweightTBL"), 6)
  }
});

</script>

<script>
import axios from "axios";

let {data} = await axios.get(
    `${import.meta.env.VITE_BACKEND_URL}/weightclass_leading_submissions/heavyweight`,
);
const leaderboardJson = data;

export default {
  async setup() {
    return {leaderboardJson};
  },
};
</script>
