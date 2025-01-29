<template>
  <div class="container mt-4 main">
    <h1>University of Liverpool &ndash; Powersort Competition</h1>

    <img class="homeicon" src="../assets/powersort.svg" alt="Powersort Competition Logo"/>
    <p>
      Help us as citizen scientist to improve <strong>Python's sorting algorithm</strong>!
      Win <strong>prizes</strong> by solving puzzles and help the climate! Every bit
      counts.
    </p>

    <p style="color: red;"><em><strong>🔥Update🔥</strong><br/>
      Round 3 of <BLink to="/aboutA">Track A</BLink> features a new Cost Measure.<br/>
      That means a new chance to win prizes!</em>
    </p>

    <h3>What's Powersort?</h3>
    <p><strong>Powersort</strong> is a refinement of <strong>Timsort</strong>,
      the sorting algorithm developed by Tim Peters for the CPython reference
      implementation of Python.
      Powersort is now used in both CPython and PyPy.
    </p>
    <p>
      Sorting is a core step in many computational tasks, and a faster sorting algorithm
      can provide faster
      execution time and reduced energy consumption without changes to application code.
      Based on our research, sorting lists in Python is up to
      <BLink
          href="https://news.liverpool.ac.uk/2022/12/12/liverpool-computer-scientists-improve-python-sorting-function/"
          target="_blank" rel="noopener">
        40% faster since Python 3.11
      </BLink>
      due to Powersort.
      To learn more about the algorithm itself, check out the
      <BLink href="https://www.youtube.com/watch?v=XjOnY-OLAPc"
             target="_blank" rel="noopener">PyCon US talk about Powersort.
      </BLink>
    </p>
    <p>
      Help us to better understand the algorithm's performance in the wild, and bring its
      benefits to other communities.
    </p>
    <p>
      <em>And earn some serious <strong>prizes</strong> along the way! 💸</em>
    </p>

    <h2>The Powersort Competition</h2>
    <p>
      We seek your help in 3 tracks:
    </p>
    <ul>
      <li>
        <BLink to="/aboutA">
          <font-awesome-icon icon="puzzle-piece" fixed-width/>
          Track A: <em>Sort this, Timsort!</em></BLink>
      </li>
      <li>
        <BLink to="/aboutB">
          <font-awesome-icon icon="database" fixed-width/>
          Track B: <em>Sorting-in-the-Wild Benchmark</em>
        </BLink>
      </li>
      <li>
        <BLink to="/aboutC">
          <font-awesome-icon icon="code-merge" fixed-width/>
          Track C: <em>Commit (to) the future of sorting!</em>
        </BLink>
      </li>
    </ul>

    <p>
      <strong>Track B</strong> requires <strong>no algorithmic knowledge</strong>;
      you just run your favorite Python project on our virtual machine
      and donate the anonymous performance data to science!
    </p>
    <p>
      <strong>Track A</strong> requires coming up with some <strong>lists to sort</strong>,
      with the goal to find lists where Timsort and Powersort differ the most.
      We have a demo notebook to get you started and you do not need to have a
      detailed understanding of the algorithms to win (although it doesn't hurt either
      😉).
    </p>
    <p>
      <strong>Track C</strong> is for the hands-on <strong>developers</strong>
      who want to bring about change with production code.
      Track C is a bounty for any accepted <strong>pull requests</strong>
      that bring Powersort to a software framework not currently using it,
      as an incentive to invest time on fundamental algorithms that help everyone.
      Your contribution will be recognized and publicized on our 
      website and in our reports.
    </p>

    <p>Head over to the detailed instructions about the tracks &ndash;
      <BLink to="/aboutA">Track&nbsp;A</BLink>
      ,
      <BLink to="/aboutB">Track&nbsp;B</BLink>
      ,
      <BLink to="/aboutC">Track&nbsp;C</BLink>
      ,
      &ndash;
      or the
      <BLink to="/submission">submission page</BLink>
      if you're ready to submit!
      Check out how others are doing on the
      <BLink to="/stats">leaderboard.</BLink>

    </p>

    <h3>Background</h3>
    <p>
      <!--      “You can't sort this list (with Timsort as fast as you should)”-->

      Timsort and Powersort are <a href="https://en.wikipedia.org/wiki/Adaptive_sort"><em>adaptive</em>
      sorting algorithms</a>:
      both are faster if the input has long presorted areas (“runs”).
      However, they differ in the <strong>merge policy</strong>, i.e.,
      the order in which they combine these naturally occurring into
      longer runs.</p>
    <p>
      Powersort solves this task of finding good merge trees
      by implicitly solving an optimization problem looking for a nearly
      <a target="_blank" rel="noopener" href="https://en.wikipedia.org/wiki/Optimal_binary_search_tree">optimal binary
        search tree</a>!
    </p>
    <h4 style="padding-top:2ex;">
      <font-awesome-icon icon="address-book" name="contact"/>
      Contact
    </h4>
    <p>
      If you have any questions, comments, or feedback, please contact us at
      <a href="mailto:powersort@liverpool.ac.uk"> <font-awesome-icon icon="envelope"/>&nbsp;<code>powersort@liverpool.ac.uk</code></a>.
    </p>
    <p>
      The Powersort Competition is organized by
      <a href="https://www.wild-inter.net" target="_blank" rel="noopener">Sebastian
        Wild</a> and kindly sponsored by
      the <a href="https://liverpool.ac.uk" target="_blank" rel="noopener">University of
      Liverpool</a> as part of
      the research impact acceleration programs.
    </p>

    <br/>
    <!--
        <Suspense>
          <Leaderboard/>
        </Suspense>
    -->
  </div>
</template>

<script setup>
//import Leaderboard from "@/components/Leaderboard.vue";
import {BLink} from "bootstrap-vue-next";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

</script>
