<template>
  <div class="useful">
    <h2>"What file formats can I use for the submission?"</h2>
    <p>
      Currently, the only accepted file format is a plain text file.
      The contents of the files should be a list of comma separated elements,
      with the list enclosed in square brackets. For example, the following
      are valid contents of a file:

      <br/>
      <br/>
      <code> [1, 'b', -1025, 3, 'a', 5] </code>
    </p>

    Notice that valid elements are those that have intrinsic lexicographic ordering.

    <br/>
    <br/>
    <h2>"What is the maximum file size allowed?"</h2>
    <p>
      There is no upper limit on the file size. However, as of right now, the number
      of elements should not exceed <code>TODO</code> elements, due to limitations
      of in-browser resource allocation towards computing the costs of Timsort and
      Powersort.

      <br/>
      <br/>
      This will, however, change in the upcoming days, with an upper limit of <code>15^6</code>
      elements being feasible.
    </p>

    <br/>
    <br/>
    <h2>"How can I start? I need some inspiration!"</h2>
    <p>

    </p>

  </div>
</template>
<script setup lang="ts"></script>
