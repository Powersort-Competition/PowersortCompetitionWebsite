<template>
  <div class="container mt-4 main">

    <h1>
      <font-awesome-icon icon="puzzle-piece"/>
      Track A: Sort this, Timsort!
    </h1>
    <p>
      In this <strong>exploratory data puzzle</strong>, your goal is to find lists for
      Timsort and Powersort where they differ the most.
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
      <a href="https://en.wikipedia.org/wiki/Optimal_binary_search_tree">optimal binary
        search tree</a>!
    </p>
    <p>
      In Track A, your goal is to find inputs for which the <strong>difference in
      efficiency</strong>
      between the merge policies of <strong>Timsort and Powersort</strong> is as big as
      possible.
    </p>
    <p>
      Timsort has some known blind spots where it performs poorly
      (see <span v-katex="'\\mathcal R_{\\text{tim}}'"></span> from Theorem 3 of <a
        href="https://arxiv.org/pdf/1801.04641">Buss and Knop (2018)</a>.
    </p>
    <p>
      <em>Can you find more?</em>
    </p>


    <h2>Track A Instructions</h2>
    <p>
      Your task is simply to find orders of input lists, where the merge policy of
      Timsort and Powersort have different cost.
      Each submission is a text file with a single list of elements, separated by commas
      (a Python list expression), e.g., “<code>[11, 12, 13, 14, 1, 2, 3]</code>”.
    </p>
    <p>If you can't get your submissions to work, check out the
      <BLink to="/useful">FAQ</BLink> or contact us.
    </p>

    <h3>Cost Measures</h3>
    <p>Comparisons and merge cost.</p>
    <h3>Weight (List Length) Classes </h3>
    <!--
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
    -->
    The competition is split into three discrete classes - <i>flyweight</i>, <i>mediumweight</i>
    and <i>heavyweight</i>.
    Each submission is allocated a specific class based on its number of elements. As
    such, we aim to capture anything
    from edge cases in small inputs that might cause Timsort to perform greater than
    Powersort, to larger and more
    complex inputs.

    <br>
    <br>
    <BCardGroup deck>
      <BCard bg-variant="light" header="Flyweight" class="text-center">
        <BCardText>Submissions where the number of elements is less than
          <code>10,000</code>.
        </BCardText>
      </BCard>
      <BCard bg-variant="light" header="Mediumweight" class="text-center">
        <BCardText>Submissions where the number of elements is at least
          <code>10,000</code>
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
    There are prizes to be won for each submission class. You are welcome to submit as
    many times as you like, and for
    any of the class!

    <h2>Prizes</h2>

    <PrizeTable/>
  </div>
</template>

<script setup>
import {BCard, BCardGroup, BCardText, BLink} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
</script>
