<template>
  <div class="container mt-4 main">
    <h1>University of Liverpool &ndash; Powersort Competition</h1>

    <img class="homeicon" src="../assets/powersort.svg" alt="Powersort Competition Logo"/>
    <p>
      Help us as citizen scientist to improve <strong>Python's sorting algorithm</strong>!
      Win <strong>prizes</strong> by solving puzzles and help the climate! Every bit
      counts.
    </p>
    <h3>What's Powersort?</h3>
    <p>
      Sorting is a core step in many computational tasks, and a faster sorting algorithm
      can provide faster
      execution time and reduced energy consumption without changes to application code.
      Based on our
      <BLink variant="primary" href="https://www.youtube.com/watch?v=XjOnY-OLAPc"
             target="_blank" rel="noopener">
        theoretical computer science research
      </BLink>
      ,
      sorting lists in Python is up to
      <BLink
          href="https://news.liverpool.ac.uk/2022/12/12/liverpool-computer-scientists-improve-python-sorting-function/">
        40% faster since Python 3.11
      </BLink>
      due to Powersort.
    </p>
    <p>
      Help us better understand the algorithm's performance in the wild, and bring its
      benefits to other communities.
      And earn some serious prizes along the way!
    </p>
    <h2>The Competition</h2>
    <p>
      We seek your help in 3 tracks:
    </p>
    <ul>
      <li>
        <font-awesome-icon icon="puzzle-piece" fixed-width/>
        Track A: Exploratory data puzzle
      </li>
      <li>
        <font-awesome-icon icon="database" fixed-width/>
        Track B: Contribute to a sorting benchmark
      </li>
      <li>
        <font-awesome-icon icon="code-merge" fixed-width/>
        Track C: Implementation sprint for Powersort in software frameworks
      </li>
    </ul>

    <p>
      Track B requires no algorithmic knowledge; you just run your favorite Python project
      on our virtual machine.
      Track A requires some tinkering with the algorithms, but we have a demo notebook to
      get you started.
      Track C is for developers who want to get their hands dirty with production code.
    </p>
    <h4>
      <font-awesome-icon icon="puzzle-piece"/>
      Track A: Sort this, Timsort!
    </h4>
    <p>
      In this <strong>exploratory data puzzle</strong>, your goal is to find lists for
      Timsort and Powersort where they differ the most.
    </p>
    <p>
      <!--      “You can't sort this list (with Timsort as fast as you should)”-->

      Timsort and Powersort are <a href="https://en.wikipedia.org/wiki/Adaptive_sort"><em>adaptive</em>
      sorting algorithms</a>:
      they are faster if the input has more presorted areas (“runs”).
      This behavior is based on finding good merge trees
      (by implicitly solving an optimization problem looking for a nearly
      <a href="https://en.wikipedia.org/wiki/Optimal_binary_search_tree">optimal binary
        search tree</a>!)

      In Track A, your goal is to find inputs for which the <strong>difference in
      efficiency</strong>
      between the merge policies of <strong>Timsort and Powersort</strong> is as big as
      possible.
    </p>
    <p>
      Timsort has some known blind spots where it performs poorly
      (see <span v-katex="'\\mathcal R_{\\text{tim}}'"></span> from Theorem 3 of <a
        href="https://arxiv.org/pdf/1801.04641">[BK2018]</a>.
    </p>
    <p>
      <em>Can you find more?</em>
    </p>
    <h4>
      <font-awesome-icon icon="database"/>
      Track B: Sorting-in-the-Wild Benchmark
    </h4>
    <p>
      Crowdsourced benchmark

      We want to understand sorting performance in your applications.
    </p>

    <h4>
      <font-awesome-icon icon="code-merge"/>
      Track C: Commit (to) the future of sorting!
    </h4>
    <p>Implementation sprint</p>


    <p>
      Please read the
      <BLink to="about">about</BLink>
      page to learn about the different competition tracks; what they consist of,
      and the prizes to be won!
    </p>
    <br/>
    <Suspense>
      <Leaderboard/>
    </Suspense>
  </div>
</template>

<script setup>
import Leaderboard from "@/components/Leaderboard.vue";
import {BLink} from "bootstrap-vue-next";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

</script>
