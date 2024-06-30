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

      Timsort and Powersort are <a target="_blank" rel="noopener" href="https://en.wikipedia.org/wiki/Adaptive_sort"><em>adaptive</em>
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
    <p>
      In Track A, your goal is to find inputs for which the <strong>difference in
      efficiency</strong>
      between the merge policies of <strong>Timsort and Powersort</strong> is as big as
      possible.
    </p>
    <p>
      Timsort has some known blind spots where it performs poorly
      (see <span v-katex="'\\mathcal R_{\\text{tim}}'"></span> from Theorem 3 of <a
        href="https://arxiv.org/pdf/1801.04641" target="_blank" rel="noopener">Buss and Knop (2018)</a>.
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
      <BLink to="/useful">FAQ</BLink>
      or
      <BLink href="mailto:powersort@liverpool.ac.uk">contact us.</BLink>
    </p>
    <p>
      Additionally, you can watch a video walkthrough of Track A highlighting some different (creative!) ways you can
      generate inputs for the competition, and how the submission process itself works:
      <BButton @click="videoModal = !videoModal"><font-awesome-icon icon="video"/> Track A walkthrough</BButton>
      <BModal v-model="videoModal" title="Track A video walkthrough"><TrackAVideo /></BModal>
    </p>

    <h3>Performance Metric</h3>
    <p>For a given list, we will use the relative cost as the goal:</p>
    <p>
      <span
          v-katex="'\\dfrac{\\text{cost}(\\text{Timsort})}{\\text{cost}(\\text{Powersort})}'"></span>

    </p>
    <p>
      The larger this ratio, the better your submission.
    </p>
    <p>
      The competition is split into 6 <strong>subcompetitions</strong>: 3 size categories
      and 2
      cost measures.
      Your submission is automatically assigned its weight class and always participates
      in the competitions for both cost measures.
    </p>
    <p>
      <em>There are prizes to be won for each subcompetition!</em>
      <br/>
      You are welcome to submit as
      many inputs as you like, and for
      any of the classes!
    </p>


    <h4>Cost Measures</h4>
    <p>The cost of sorting a list can be measured in many ways; we focus on the
      following.</p>
    <p>
      <BCardGroup deck>
        <BCard bg-variant="light" header-html="<strong>Comparisons</strong>">
          <BCardText>
            The number of times two elements are <strong>compared</strong> to each other.
          </BCardText>
        </BCard>
        <BCard bg-variant="light" header-html="<strong>Merge Cost</strong>">
          <BCardText>
            The total <strong>merge cost</strong>, where the merge cost of merging
            two runs is the sum of their lengths.
          </BCardText>
        </BCard>
      </BCardGroup>
    </p>
    <!--
        <ul>
          <li>the number of <strong>comparisons</strong> (the number of times two elements are
            compared to
            each other), and
          </li>
          <li>the total <strong>merge cost</strong>, where the merge cost of a single merge
            of two runs is the sum of their lengths.
          </li>
        </ul>
    -->
    <p>
      Comparisons often dominate the running time of sorting in CPython since
      they can require executing interpreted user Python code.
      Merge cost is proportional to the number of elements moved around in memory and
      hence can be a good proxy for cache misses and energy consumption.
    </p>

    <h4>Weight (List Length) Classes </h4>
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
    <p>
      The competition is split into three discrete classes &ndash; <i>flyweight</i>, <i>mediumweight</i>
      and <i>heavyweight</i>.
      Each submission is allocated a specific class based on its number of elements. As
      such, we aim to capture anything
      from edge cases in small inputs that might cause Timsort to perform greater than
      Powersort, to larger and more
      complex inputs.
    </p>
    <p>
      <BCardGroup deck>
        <BCard bg-variant="light" header="Flyweight">
          <BCardText>Submissions where the number of elements is less than
            <code>10,000</code>.
          </BCardText>
        </BCard>
        <BCard bg-variant="light" header="Mediumweight">
          <BCardText>Submissions where the number of elements is at least
            <code>10,000</code>
            and at most <code>1,000,000</code>.
          </BCardText>
        </BCard>
        <BCard bg-variant="light" header="Heavyweight">
          <BCardText>Submissions where the number of elements is equal to or greater than
            <code>1,000,000</code>.
          </BCardText>
        </BCard>
      </BCardGroup>
    </p>


    <h2>Prizes</h2>
    <p>
      The competition will proceed in rounds of 4 weeks each.
      Every participant who beat a previous record in any of the 6 subcompetitions
      in that round will get a prize (up to the maximum available prize money of £2500 for
      Track A).
    </p>
    <p>
      There are 3 <strong>classes of prizes</strong>, 1st, 2nd, and 3rd prizes,
      which are awarded according to the <strong>relative improvement</strong>
      of a submitted input over the previous record in the same subcompetition.
    </p>
    <PrizeTable/>
    <p>
      The length of rounds is subject to change.
    </p>
  </div>
</template>

<script setup>
import {BCard, BCardGroup, BCardText, BLink, BModal, BButton} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
import {ref} from "vue";

const videoModal = ref(false);
</script>
