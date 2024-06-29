<template>
  <div class="container mt-4 main">
    <h1><font-awesome-icon icon="puzzle-piece"/> Track A Submission</h1>
    <p>
      For Track A, a submission consists of a single text file containing a list of
      elements, separated by commas (a Python list expression), e.g.,
      <code>[11, 12, 13, 14, 1, 2, 3]</code>.
    </p>
    <p>
      (For more information, see the instructions on the <BLink to="/aboutA">Track A</BLink> page.)
    </p>
    <div v-if="fileDropComponent">
      <div v-if="processed == false">
        <FileDropper @file-dropped="handleFileDrop"/>
      </div>
      <div v-else-if="needsServerComp == true">
        Your submission is too large for in-browser computation, so it has been
        sent to the server for processing. You will be notified by email the
        results.
      </div>
      <div v-else-if="processed == true">
        <BAlert :model-value="true" variant="success">
          <h4 class="alert-heading">
            <IMdiCheck/>
            Success!
          </h4>

          <p>
            Your submission has been computed and recorded successfully! You should momentarily
            receive an email receipt outlining computation details for your convenience.

            <br>
            It is now safe to close this page.
          </p>

          <hr>

          <p class="mb-0">
            <b>Number of Powersort Comparisons</b>: <code>{{ psortComps }}</code>
            <br>
            <b>Number of Timsort Comparisons</b>: <code>{{ tsortComps }}</code>
            <br>
            <b>Powersort MergeCost</b>: <code>{{ psortMergeCost }}</code>
            <br>
            <b>Timsort MergeCost</b>: <code>{{ tsortMergeCost }}</code>
          </p>
        </BAlert>
      </div>
    </div>
  </div>
</template>

<script setup>
import axios from "axios";
import FileDropper from "@/components/FileDropper.vue";
import {nextTick, ref} from "vue";
import {asyncRun} from "../py_webworker.js";
import {getEmailFromCookie, getInputSize, getUserID, percDifference} from "@/misc.js";

import {BAlert} from "bootstrap-vue-next";
import router from "@/router/index.js";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

// $cookies.set("pscomp_oauth", JSON.stringify({"email": "shayan.doust@outlook.com"}));
// Check if oauth cookie is set. If not, redirect to login.
if ($cookies.get("pscomp_oauth") == null) {
  console.log("Not logged in... routing to login page");
  router.push({name: "login"});
}

let backendOnline = false;
let needsServerComp = false;
let processed = false;
let psortComps, tsortComps, psortMergeCost, tsortMergeCost;

// if (await backendHealthCheck() === "pong") { backendOnline = true; } else { backendOnline = false; }

const fileDropComponent = ref(true);
const email = getEmailFromCookie();
let userID = getUserID(email);

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


  const servResponse = ref(null);
  var submission_input_data = new FormData();

  submission_input_data.append("file", submission_content);

  // If input file is too big for Pyodide, send to server for computation instead.
  if (getInputSize(submission_content) <= 5_000_000) {
    //pyodide.FS.writeFile("./submission.txt", submission_content, { encoding: "utf8" });
    let comps = await runPyWebWorker(submission_content);

    psortComps = comps.results[0].get("Comparisons");
    tsortComps = comps.results[1].get("Comparisons");
    psortMergeCost = comps.results[0].get("MergeCost");
    tsortMergeCost = comps.results[1].get("MergeCost");
  } else {
    console.log("File too big for Pyodide, sending to server.");

    needsServerComp = true;

    axios.post("/serverside_comp", submission_input_data, {
      headers: {
        "user-id": "test",
        "Access-Control-Allow-Origin": "*",
      }
    })
  }

  // Now that we have the comparison counts (from in-browser computation), send to server.
  let requestData = {
    user_id: await userID,
    powersort_comp: psortComps,
    timsort_comp: tsortComps,
    perc_diff: percDifference(tsortMergeCost, psortMergeCost),
    powersort_merge_cost: psortMergeCost,
    timsort_merge_cost: tsortMergeCost,
    submission_size: getInputSize(submission_content),
  };
  axios.post("/new_submission_track_a", requestData, {
    headers: {
      "content-type": "application/json",
      "Access-Control-Allow-Origin": "*",
    }
  }).then((response) => {
    axios.post("/submission_input_save", submission_input_data, {
      headers: {
        "file-name": response.data,
        "track": "A",
        "Access-Control-Allow-Origin": "*",
      }
    })
  });

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
