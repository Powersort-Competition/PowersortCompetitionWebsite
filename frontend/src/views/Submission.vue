<template>
  <div class = "submission">
    <h1>Submission</h1>

  </div>
</template>

<script setup>
import router from '../router/index.js'

// Check if oauth cookie is set. If not, redirect to login.
if ($cookies.get('pscomp_oauth') == null)
{
	console.log("Not logged in... routing to login page");
	router.push({ name: 'login' })
}
/**
import { loadPyodide } from "pyodide";

let pyodide = await loadPyodide({
  indexURL: 'https://cdn.jsdelivr.net/pyodide/v0.25.0/full/'
});

pyodide.runPythonAsync(`
from pyodide.ffi import to_js
from pyodide.http import pyfetch
response = await pyfetch("https://corsproxy.io/?https%3A%2F%2Fgithub.com%2Fshayandoust%2FPowersortCompetitionWebsite%2Fraw%2Fmain%2Ffrontend%2Fpy_assets%2Fpowersort_timsort.tar.gz") # .zip, .whl, ...
await response.unpack_archive() # by default, unpacks to the current dir

import Powersort as Powersort
import Timsort as Timsort
import Counters as Counters

def cost(lst, sorter):
    wrapped = [Counters.ComparisonCounter(x) for x in lst]
    Counters.reset_counters()
    sorter.sort(wrapped)
    assert Counters.ComparisonCounter.EQ_COMPARISONS == 0

    return {
        'Algorithm': sorter.name(),
        'Comparisons': Counters.ComparisonCounter.COMPARISONS
    }

def compare_sorters(lst, sorters = [Powersort, Timsort]):
    sorters = sorted(sorters, key = lambda sorter: sorter.name())

    return to_js([cost(lst, sorter) for sorter in sorters])
`)

// function processFileSubmission()
// {
//   var file = document.getElementById("file").files[0];
//   var reader = new FileReader();
//   reader.readAsBinaryString(file);
//
//   reader.onload = event =>
//   {
//     file_content = event.target.result;
//     pyodide.runPython(`from js import file_content\nprint(compare_sorters(file_content))`);
//
//   }
// }
**/
</script>
