<template>
  <div class="container mt-4 main">
    <h4>
      <font-awesome-icon icon="question-circle"/>
      What file formats can I use for the submission?
    </h4>
    <p>
      Currently, the only accepted file format is a plain text file.
      The contents of the files should be a list of comma separated elements,
      with the list enclosed in square brackets. For example, the following
      are valid contents of a file:
      <br/>
      <code> [1, 'b', -1025, 3, 'a', 5] </code>
    </p>

    <p>Notice that valid elements are those that have intrinsic lexicographic
      ordering.</p>

    <h4>
      <font-awesome-icon icon="question-circle"/>
      What is the maximum file size allowed?
    </h4>
    <p>
      There is no hard limit on the file size.
      For short lists, the costs of Timsort and Powersort are computed in the browser
      using pyodide, a Python interpreter compiled to WebAssembly.
      For these, results are available immediately.
      Due to limitations of in-browser resource allocation, large inputs are
      automatically submitted to our server for processing and results will be sent
      to you via email.
    </p>
  </div>
</template>
<script setup lang="ts">
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
</script>
