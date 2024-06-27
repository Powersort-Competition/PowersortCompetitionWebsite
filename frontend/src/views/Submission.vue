<template>
  <div class="container mt-4 main">
    <h1>Submission Pages</h1>
    <p>
      Ready to contribute as citizen scientist to improve Python's sorting algorithm?
      Eager to win prizes and help the climate? Go for it! Every bit counts.
    </p>
    <p>
      Check out the
      <BLink to="about">home page</BLink>
      for instructions and select the track below.
    </p>
    <p>
      <strong>To submit to the Powersort Competition you have to
        <BLink to="login">Log In</BLink> with a Google account!
      </strong>
      We will send you a receipt email for each submission that you need to claim any prizes.
      The name associated with your Google account will be shown on the leaderboard.
    </p>
    <ul>
      <li style="margin-bottom: 1ex;">
        <BButton to="trackA" block variant="outline-primary">
          <font-awesome-icon icon="puzzle-piece" fixed-width/>
          <strong>Track A:</strong> Sort this, Timsort!
        </BButton>
      </li>

      <li style="margin-bottom: 1ex;">
        <BButton to="trackB" block variant="outline-primary">
          <font-awesome-icon icon="database" fixed-width/>
          <strong>Track B:</strong> Sorting-in-the-Wild Benchmark
        </BButton>
      </li>

      <li>
        <BButton to="trackC" block variant="outline-primary">
          <font-awesome-icon icon="code-merge" fixed-width/>
          <strong>Track C:</strong> Commit (to) the future of sorting!
        </BButton>
      </li>
    </ul>
  </div>
</template>

<script setup>
import {BLink, BButton} from "bootstrap-vue-next";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
</script>
