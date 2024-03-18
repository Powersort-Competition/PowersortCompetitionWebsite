<template>
  <div class="home">
    <h1>Powersort Competition</h1>

    <br>
    <div class="leaderboard-table">
      <table class="table">
        <thead>
        <tr>
          <th scope="col">#</th>
          <th scope="col">Name</th>
          <th scope="col">Score</th>
        </tr>
        </thead>
      </table>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const leaderboard_json = ref(null);
</script>