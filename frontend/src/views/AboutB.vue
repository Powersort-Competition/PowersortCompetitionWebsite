<template>
  <div class="container mt-4 main">
    <h1>
      <font-awesome-icon icon="database"/>
      Track B: Sorting-in-the-Wild Benchmark
    </h1>
    <p>
      In Track B, you will contribute towards a <strong>benchmark</strong> to evaluate the
      performance of adaptive sorting algorithms such as Timsort and Powersort in
      various applications.
    </p>
    <p>
      A valid submission for Track B consists of a log file <code>arrays.txt</code>
      produced by our instrumented CPython version and description of how to reproduce
      the data. For example, you could give a link to the repository of the project
      you ran and the command, parameters, and data you used.
    </p>
    <p>
      The goal is to find Python applications were many and long lists are sorted using
      the built-in <code>sorted</code> function or the <code>sort</code> method of lists.
    </p>
    <p>
      <em>Any Python application is welcome; the more diverse the data, the better!</em>
      <br/>
      However, scripts that simply sort random or synthetic data are not eligible
      for Track B; the application or script has to serve a purpose beyond sorting.
    </p>
    <h3>Instructions</h3>
    <p>You can obtain a VBox virtual-machine image that has our instrumented CPython
      clone already installed and ready to use.
      In the virtual machine, you simply install your Python application as you would
      on your local machine and run it.
      Our custom CPython version will log the arrays that are sorted (more precisely,
      the relative order of items in the list) and log this information to a file
      <code>arrays.txt</code>.
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
import {BCard, BCardGroup, BCardText, BLink} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
</script>
