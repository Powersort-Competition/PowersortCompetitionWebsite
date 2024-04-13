<template>
  <div class="useful">
    <h2>"What file formats can I use for the submission?"</h2>
    <p>
      Currently, the only accepted file format is a plain text file with the extension ".txt".
      The contents of the files should be a list of comma separated elements, with the list enclosed
      in square brackets. For example, the following are valid contents of a file:

      <br> <br>
      <code>
        [1, 2, 3, 4, 5]
      </code>
    </p>

    <br> <br>
    <h2>"What is the maximum file size allowed?"</h2>
    <p>
      todo
    </p>

    <br><br>
    <h2>"I need some inspiration for an input..."</h2>
    <p>
      Whilst you can generate an input of random permutations and hope it performs worse on Powersort
      than Timsort, one can be more strategic in their approach. For instance, one can pick permutations
      from some probabilistic distribution that can be hard for Powersort, whilst another way could be
      using real-world data such as
      <a class="btn btn-outline-success btn-sm" href="https://github.com/mxw/grmr/blob/master/src/finaltests/bible.txt" role="button">
        The Bible
      </a>.

      <br> <br>
      Regardless, some examples exist in the
      <a class="btn btn-outline-success btn-sm" href="https://github.com/shayandoust/PowersortCompetitionExamples" role="button">
        Powersort Competition Examples
    </a> repository.
    </p>
  </div>
</template>
<script setup lang="ts">
</script>