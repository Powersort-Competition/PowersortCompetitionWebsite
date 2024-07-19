<template>
  <div class="container mt-4 main">
    <h1>
      <font-awesome-icon icon="code-merge"/>
      Track C: Commit (to) the future of sorting!
    </h1>
    <p>
      In Track C, you will contribute towards the <strong>future</strong> of sorting by
      integrating Powersort into a software framework that does not currently use it.
    </p>
    <p>Ready to submit? &nbsp;
      <BButton to="trackC" block variant="outline-primary">
        <font-awesome-icon icon="code-merge" fixed-width/>
        <strong>Track C:</strong> Commit (to) the future of sorting!
      </BButton>
    </p>
    <p>
      A valid submission for Track C consists of a link to the <strong>public pull
      request</strong> on GitHub or another public code hosting platform where you have
      implemented Powersort.
      You will have to provide a brief description of the software framework and the
      changes you made to integrate Powersort, as well as what sorting algorithms were
      used before.
      The submission also has to prove that you have contributed to the pull request
      yourself
      in a substantial way.
    </p>
    <p>
      To be eligible for a prize in Track C, pull requests have to be accepted by the
      maintainers of project, and the project has to serve a plausible purpose
      (beyond sorting), e.g., demonstrated by its use in other project.
    </p>
    <h3>How to get started</h3>
    <p>
      Some simple <strong>reference implementations</strong> of Powersort are available in
      the following repositories under a permissive license (MIT software license) that
      you can use as a starting point:
    </p>
    <ul>
      <li>
        <BLink
            href="https://github.com/sebawild/powersort/blob/main/src/sorts/powersort.h">
          C++ implementation
        </BLink>
        (from the <a
          href="https://www.wild-inter.net/publications/cawley-gelling-nebel-smith-wild-2023"><em>Multiway
        Powersort</em></a> paper)
      </li>
      <li>
        <BLink
            href="https://github.com/sebawild/nearly-optimal-mergesort-code/blob/master/src/wildinter/net/mergesort/PowerSort.java">
          Java implementation
        </BLink>
        (from the <a href="https://www.wild-inter.net/publications/munro-wild-2018">original
        Powersort paper</a>)
      </li>
    </ul>
    <p>
      Apart from these, there are the existing implementation in production that can
      serve as inspiration; (it is your responsibility to check the projects' licenses
      for terms of use):
    </p>
    <ul>
      <li>The <a href="https://github.com/python/cpython/blob/main/Objects/listobject.c">CPython
        implementation</a>
        (main function <code>list_sort_impl</code>) is written in C.
      </li>
      <li>
        The <a
          href="https://foss.heptapod.net/pypy/pypy/-/blob/branch/default/rpython/rlib/listsort.py">PyPy
        implementation</a>
        (main method <code>sort</code>) is written in RPython (≈ Python 2) and is very
        readable.
      </li>
      <li>The <a
          href="https://github.com/AssemblyScript/assemblyscript/blob/main/std/assembly/util/sort.ts">AssemblyScript
        implementation</a> (main function <code>SORT</code>) is written in TypeScript and quite
        compact and readable,
        too.
      </li>
    </ul>


    <h2>Prizes</h2>
    The prizes for Track C are two <em>10.9" Apple iPads 10th Gen</em>.
    The winners will be individually selected among all submission by the academic
    organizers.

  </div>
</template>

<script setup>
import {BButton, BCard, BCardGroup, BCardText, BLink} from "bootstrap-vue-next";

import PrizeTable from "@/components/PrizeTable.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
</script>
