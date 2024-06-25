<template>
  <div class="usefulA">
    <h1>Submission Pages</h1>
    <p>
      Help us as citizen scientist to improve Python's sorting algorithm!
      Win prizes by solving puzzles and help the climate! Every bit counts.
    </p>
    <p>
      Please read the
      <BLink to="about">about</BLink>
      page to learn about the different competition tracks; what they consist of,
      and the prizes to be won!
    </p>
    <br/>
    <p>
      <strong>To submit to the Powersort Competition, please select a track below:</strong>
      <ul>
        <li>
          <BLink to="trackA"><strong>Track A:</strong> Sort this, Timsort!</BLink>
        </li>
        <li>
          <BLink to="trackB"><strong>Track B:</strong> Sorting-in-the-Wild Benchmark</BLink>
        </li>
        <li>
          <BLink to="trackC"><strong>Track C:</strong> Commit (to) the future of sorting!</BLink>
        </li>
      </ul>
    </p>
  </div>
</template>

<script setup>
import {BLink} from "bootstrap-vue-next";
</script>
