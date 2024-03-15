<template>
  <div class = "submission">
    Hello!

  </div>
</template>

<script setup>
import { loadPyodide } from "pyodide";

let pyodide = await loadPyodide({
  indexURL: 'https://cdn.jsdelivr.net/pyodide/v0.25.0/full/'
});

await pyodide.loadPackage('micropip')
const micropip = pyodide.pyimport('micropip')
await micropip.install('requests')

pyodide.runPythonAsync(`
from pyodide.http import pyfetch
response = await pyfetch("https://corsproxy.io/?https%3A%2F%2Fgithub.com%2Fshayandoust%2FPowersortCompetitionWebsite%2Fraw%2Fmain%2Ffrontend%2Fpy_assets%2Fpowersort_timsort.tar.gz") # .zip, .whl, ...
await response.unpack_archive() # by default, unpacks to the current dir

from Powersort import sort
print(sort([1, -1, 1909, 3232, -1212, 1]))
`)

</script>