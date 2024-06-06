<template>
  <Suspense>
    <table class="table">
      <caption>
        Top 5 flyweight submissions
      </caption>
      <thead>
      <tr>
        <th scope="col">Submission ID</th>
        <th scope="col">Powersort Comparisons</th>
        <th scope="col">Timsort Comparisons</th>
        <th scope="col">Powersort Merge Cost</th>
        <th scope="col">Timsort Merge Cost</th>
        <th scope="col">Comparison ratio</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="submission in leaderboardJson">
        <td>{{ submission.submission_id }}</td>
        <td>{{ submission.powersort_comp }}</td>
        <td>{{ submission.timsort_comp }}</td>
        <td>{{ submission.powersort_merge_cost }}</td>
        <td>{{ submission.timsort_merge_cost }}</td>
        <td>{{ submission.ratio_comp }}</td>
      </tr>
      </tbody>
    </table>
  </Suspense>
</template>

<script>
import axios from "axios";

let {data} = await axios.get(
    `${import.meta.env.VITE_BACKEND_URL}/weightclass_leading_submissions/flyweight`,
);
const leaderboardJson = data;

export default {
  async setup() {
    return {leaderboardJson};
  },
};
</script>
