<template>
  <Suspense>
    <div class="wide-table" >
      <table class="table">
        <caption>
          Top 5 global submissions
        </caption>
        <thead>
        <tr>
          <th scope="col">Submission ID</th>
          <th scope="col">Submitter</th>
          <th scope="col">Powersort Comparisons</th>
          <th scope="col">Timsort Comparisons</th>
          <th scope="col">Powersort Merge Cost</th>
          <th scope="col">Timsort Merge Cost</th>
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
          <td>{{ submission.perc_diff }}</td>
        </tr>
        </tbody>
      </table>
    </div>
  </Suspense>
</template>

<script>
import axios from "axios";

// SW: My quick hack to let the leaderboard not crash the rendering; TODO proper fix
let data;
try {
  let {data} = await axios.get(
      `${import.meta.env.VITE_BACKEND_URL}/weightclass_leading_submissions/global`,
  );

} catch (e) {
  console.error(e);
  data = [];
}
const leaderboardJson = data;

export default {
  async setup() {
    return {leaderboardJson};
  },
};
</script>

<style scoped>
table th {
  background-color: transparent;
}
table td {
  background-color: transparent;
}
</style>
