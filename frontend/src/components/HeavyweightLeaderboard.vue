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
      <tr v-for="submission in leaderboardJson" :key="submission.submission_id">
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
import {defineProps, watch, ref, onMounted} from "vue";
import {sortTable, getLeaderboardJSONs} from "@/misc.js"

let mcostDiffSorted = ref([]);
let compDiffSorted = ref([]);
const leaderboardJson = ref([]);

const fetchData = async () => {
  const [mcostData, compData] = await getLeaderboardJSONs("heavyweight");
  mcostDiffSorted.value = mcostData;
  compDiffSorted.value = compData;

  leaderboardJson.value = compDiffSorted.value;
}

// Fetch data when the component is mounted.
onMounted(() => {
  fetchData();
});

const props = defineProps({
  selectedMetric: String,
  default: "comp_diff"
});

watch(() => props.selectedMetric, (newType) => {
  if (newType === "merge_diff") {
    leaderboardJson.value = mcostDiffSorted.value;
  } else {
    leaderboardJson.value = compDiffSorted.value;
  }
});

</script>
