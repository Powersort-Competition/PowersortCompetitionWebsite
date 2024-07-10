<template>
  <div class="container mt-4 main">
    <h1>
      <font-awesome-icon icon="database"/>
      Track B: Sorting-in-the-Wild Benchmark
    </h1>
    <p>
      In Track B, you will contribute towards a public-domain
      <strong>adaptive-sorting benchmark</strong>
      to evaluate the
      effectiveness of sorting algorithms such as Timsort and Powersort in
      various applications.
    </p>
    <p>Ready to submit? &nbsp;
      <BButton to="trackB" block variant="outline-primary">
        <font-awesome-icon icon="database" fixed-width/>
        <strong>Track B:</strong> Sorting-in-the-Wild Benchmark
      </BButton>
    </p>
    <h3>Background</h3>
    <p>
      The goal of Track B is to collect data on how sorting is used in real-world
      applications.
      This data will be used to evaluate the effectiveness of adaptive sorting algorithms
      such as Timsort and Powersort in various applications, and will also
      be made available under a creative-commons license to the research community
      for testing the effectiveness of any future algorithms.
    </p>
    <p>
      The benchmark will consist of lists of integers representing the relative order
      of elements sorted in real-life applications. More precisely, we only keep
      the <em>ranks</em> of the elements in the lists to anonymize the data;
      the ranks are sufficient to determine the number of comparisons and the merge cost
      of adaptive sorting algorithms.
    </p>
    <p>
      We are interested in any application were many and long lists are sorted using
      the built-in <code>sorted</code> function or the <code>sort</code> method of lists.
      <br/>
      <em>Any Python application is welcome; the more diverse the data, the better!</em>
    </p>
    <p>
      Note that scripts that simply sort random or synthetic data are not eligible
      for Track B; the application or script has to serve a plausible
      purpose beyond sorting.
    </p>
    <h2>Track B Instructions</h2>
    <p>
      A valid submission for Track B consists of a log file <code>arrays.txt</code>
      produced by our
      <a href="https://github.com/sebawild/cpython/tree/3.11-instrumented" target="_blank"
         rel="noopener">
        instrumented CPython version</a>
      and description of how to reproduce
      the data. For example, you could give a link to the repository of the project
      you ran and the command, parameters, and data you used.
    </p>

    <p>You can obtain a
      <a href="https://drive.google.com/file/d/1DejZCnmeuFYHjDdIZCzisLnmQ4mJtV_q/view?usp=drive_link"
         target="_blank" rel="noopener">
        VBox virtual-machine image</a>
      that has our instrumented CPython
      clone already installed and ready to use.
      In the virtual machine, you simply install your Python application as you would
      on your local machine and run it.
      Our custom CPython version will log the arrays that are sorted (more precisely,
      the relative order of items in the list) and log this information to a file
      <code>arrays.txt</code>.
    </p>

    <p>
      Need a kickstart in inspiration or a walkthrough with an example on using the
      VBox image, and generating the <code>arrays.txt</code> file? This video might
      just help:
    </p>
    <p>
      <BButton variant="outline-primary" @click="videoModal = !videoModal">
        <font-awesome-icon icon="video"/>
        Track B walkthrough
      </BButton>
      <BModal v-model="videoModal" title="Track B video walkthrough">
        <TrackBVideo/>
      </BModal>
    </p>

    <p>If you can't get the virtual machine to work or have doubts about eligibilty,
      <BLink href="mailto:powersort@liverpool.ac.uk">reach out to us.</BLink>
    </p>
    <h2>Prizes</h2>
    <p>
      There are 3 classes of prizes, which are awarded depending
    </p>
    <ol>
      <li>
        primarily on <strong>how much</strong>
        the application relies on <strong>sorting</strong> (how many and how long lists
        are sorted), and
      </li>
      <li>second on how <strong>meaningful and widely used</strong> an application is.
      </li>
    </ol>
    <p>
      Up to the total prize pool of £2500, every meaningful submission to Track B
      will receive at least a 3rd prize.
    </p>

    <PrizeTable/>
  </div>
</template>

<script setup>
import {BButton, BLink, BModal} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
import TrackBVideo from "@/components/TrackBVideo.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
import {ref} from "vue";

const videoModal = ref(false);
</script>
