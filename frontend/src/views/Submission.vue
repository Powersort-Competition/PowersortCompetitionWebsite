<template>
  <div class="submission">
    <div v-if="fileDropComponent">
      <div v-if="processed == false">
        <FileDropper @file-dropped="handleFileDrop" />
      </div>
      <div v-else-if="needsServerComp == true">
        Your submission is too large for in-browser computation, so it has been
        sent to the server for processing. You will be notified by email the
        results.
      </div>
      <div v-else-if="processed == true">
        Your submission caused {{ psortComps }} comparisons on Powersort and
        {{ tsortComps }} comparisons on Timsort.
      </div>
    </div>
  </div>
</template>

<script setup>
import axios from "axios";

import router from "@/router/index.js";
import FileDropper from "@/components/FileDropper.vue";
import { nextTick, ref } from "vue";
import { asyncRun } from "../py_webworker.js";
import { getInputSize, getUserID, getEmailFromCookie } from "@/misc.js";

// Check if oauth cookie is set. If not, redirect to login.
if ($cookies.get("pscomp_oauth") == null) {
  console.log("Not logged in... routing to login page");
  router.push({ name: "login" });
}

let needsServerComp = false;
let processed = false;
let psortComps, tsortComps, psortMergeCost, tsortMergeCost;

const fileDropComponent = ref(true);
const email = getEmailFromCookie();

const forceRerender = async () => {
  fileDropComponent.value = false;

  await nextTick();

  fileDropComponent.value = true;
};

const handleFileDrop = async (submission_content) => {
  console.log(
    "File dropped! Processing with length: ",
    submission_content.length,
  );

  // If input file is too big for Pyodide, send to server for computation instead.
  if (submission_content.length <= 21388890) {
    //pyodide.FS.writeFile("./submission.txt", submission_content, { encoding: "utf8" });
    let comps = await runPyWebWorker(submission_content);

    psortComps = comps.results[0].get("Comparisons");
    tsortComps = comps.results[1].get("Comparisons");
    psortMergeCost = comps.results[0].get("MergeCost");
    tsortMergeCost = comps.results[1].get("MergeCost");
  } else {
    console.log("File too big for Pyodide, sending to server.");

    needsServerComp = true;
  }

  // Now that we have the comparison counts, send to server.
  const servResponse = ref(null);
  var submission_input_data = new FormData();

  submission_input_data.append("file", submission_content);
  submission_input_data.append("submissionId", 1); // Might be able to remove this now?

  let requestData = {
    user_id: getUserID(email),
    powersort_comp: psortComps,
    timsort_comp: tsortComps,
    ratio_comp: tsortComps / psortComps,
    powersort_merge_cost: psortMergeCost,
    timsort_merge_cost: tsortMergeCost,
    submission_size: getInputSize(submission_content),
  };
  axios.post("/new_submission", requestData, {
    headers: {
      "content-type": "application/json",
      "Access-Control-Allow-Origin": "*",
    }
  })

  axios.post("/submission_input_save", submission_input_data, {
    headers: {
      "file-name": 1,
      "Access-Control-Allow-Origin": "*",
    }
  })

  processed = true;
  await forceRerender();
};

const script = `
from pyodide.ffi import to_js
from pyodide.http import pyfetch

response = await pyfetch("https://raw.githubusercontent.com/Powersort-Competition/PowersortCompetitionWebsite/main/frontend/py_assets/powersort_timsort.tar.gz") # .zip, .whl, ...
await response.unpack_archive() # by default, unpacks to the current dir

print("Received and unpacked Python components successfully!")

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
        'Comparisons': Counters.ComparisonCounter.COMPARISONS,
        'MergeCost': Counters.MergeCosts.MERGECOST
    }

def compare_sorters(lst, sorters = [Powersort, Timsort]):
    sorters = sorted(sorters, key = lambda sorter: sorter.name())

    return [cost(lst, sorter) for sorter in sorters]

with open("./submission.txt", "r") as fh:
    data = fh.read()

comps = to_js(compare_sorters(data))
comps
`;

async function runPyWebWorker(submission_content) {
  console.log("Pyodide web worker initialising.");
  try {
    let results = await asyncRun(script, [submission_content]);

    if (results) {
      console.log("Pyodide web worker returned: ", results);
    } else {
      console.log("Pyodide web worker failed and returned: ", error);
    }

    return results;
  } catch (e) {
    console.log(
      `Error with Pyodide web worker at ${e.filename}, ${e.lineno}, ${e.message}`,
    );
  }
}
</script>
