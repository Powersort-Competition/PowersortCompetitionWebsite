<template>
  <div class="main">
    <h1>Background</h1>
    <p>
      Help us as citizen scientist to improve Python's sorting algorithm! Win prizes by solving puzzles and
      help the climate! Every bit counts.

      <br>
      <br>
      Sorting is a core step in many computational tasks, and a faster sorting algorithm can provide faster
      execution time and reduced energy consumption without changes to application code. Based on our
      <BLink variant="primary" href="https://www.youtube.com/watch?v=XjOnY-OLAPc" target="_blank" rel="noopener">
        theoretical computer science research
      </BLink>,
      sorting lists in Python is up to <BLink href="https://news.liverpool.ac.uk/2022/12/12/liverpool-computer-scientists-improve-python-sorting-function/">40% faster since Python 3.11</BLink> due to Powersort.

      <br>
      <br>
      Help us better understand the algorithm's performance in the wild, and bring its benefits to other communities.
      And earn some serious prizes along the way!

      <br>
      <br>
      We seek your help in 3 tracks:
    </p>

    <BCardGroup deck>
      <BCard bg-variant="light" header="Track A" class="text-center">
        <BCardText>
          <p>
            Timsort and Powersort are <i>adaptive</i> sorting algorithms: they are faster if the input has more
            presorted
            areas ("runs"). This behavior is based on finding good merge trees (by implicitly solving an optimization
            problem looking for a nearly [optimal binary search tree](CP ref geeks for geeks or so)!).

            <br>
            <br>
            In this track, your goal is to find inputs for which the difference in efficiency between the merge policies
            of Timsort and Powersort is as big as possible.

            <br>
            <br>
            Timsort has some known blind spots where it performs poorly. <b>Can you find more?</b>
          </p>
        </BCardText>
      </BCard>
      <br>
      <BCard bg-variant="light" header="Track B" class="text-center">
        <BCardText>
          <p>
            We want to understand sorting performance in your applications.
          </p>
        </BCardText>
      </BCard>
      <br>
      <BCard bg-variant="light" header="Track C" class="text-center">
        <BCardText>
          <p>

          </p>
        </BCardText>
      </BCard>
    </BCardGroup>
    <h1>Competition Details</h1>
    <h2>Track A</h2>
    The competition is split into three discrete classes - <i>flyweight</i>, <i>mediumweight</i> and <i>heavyweight</i>.
    Each submission is allocated a specific class based on its number of elements. As such, we aim to capture anything
    from edge cases in small inputs that might cause Timsort to perform greater than Powersort, to larger and more
    complex inputs.

    <br>
    <br>
    <BCardGroup deck>
      <BCard bg-variant="light" header="Flyweight" class="text-center">
        <BCardText>Submissions where the number of elements is less than <code>10,000</code>.</BCardText>
      </BCard>
      <BCard bg-variant="light" header="Mediumweight" class="text-center">
        <BCardText>Submissions where the number of elements is at least <code>10,000</code>
          and at most <code>1,000,000</code>.
        </BCardText>
      </BCard>
      <BCard bg-variant="light" header="Heavyweight" class="text-center">
        <BCardText>Submissions where the number of elements is equal to or greater than
          <code>1,000,000</code>.
        </BCardText>
      </BCard>
    </BCardGroup>

    <br>
    <br>
    There are prizes to be won for each submission class. You are welcome to submit as many times as you like, and for
    any of the class!

    <h2>Track B</h2>

    <h2>Track C</h2>

    <br>
    <br>
    <br>
    <h2>Prizes</h2>

    <PrizeTable/>
  </div>
</template>

<script setup>
import {BCard, BCardGroup, BCardText, BLink} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
</script>
