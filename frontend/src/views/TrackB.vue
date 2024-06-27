<template>
  <div class="container mt-4 main">
    <h1>Track B Submission</h1>
  <suspense>
  <div v-if="fileDropComponent">
    <div v-if="processed === false">
      <FileDropper @file-dropped="handleFileDrop"/>
      <div class="textAreaTrackBox">
        <BFormTextarea
            id="textarea"
            v-model="submissionDescription"
            placeholder="Describe how you attained your input."
            rows="3"
            class="mt-4"
        />
        <BButton class="mt-4" @click="buttonClicked">Submit</BButton>
      </div>
    </div>
    <div v-else-if="processed === true"
         style="display: flex; position: absolute; top: 20%; left: 32%; right: 32%;">
      <BAlert :model-value="true" variant="success">
        <h4 class="alert-heading">
          <IMdiCheck/>
          Success!
        </h4>

        <p>
          Your submission has been saved successfully! You should momentarily receive an email
          receipt.

          <br>
          It is now safe to close this page.
        </p>
      </BAlert>
    </div>
  </div>
    <div v-else-if="processed === true">
      Processed!
    </div>
  </suspense>
  </div>
</template>

<script setup>
import FileDropper from "@/components/FileDropper.vue";
import axios from "axios";
import {nextTick, ref} from "vue";
import {BFormTextarea, BButton, BAlert} from "bootstrap-vue-next";
import {getEmailFromCookie, getUserID} from "@/misc.js";

const fileDropComponent = ref(true);
const submissionDescription = ref();

let processed = false;
let submissionContent = null;
let userID = getUserID(getEmailFromCookie());

const forceRerender = async () => {
  fileDropComponent.value = false;

  await nextTick();

  fileDropComponent.value = true;
}

const handleFileDrop = async (content) => {
  console.log("File dropped!");

  submissionContent = content;
}

const buttonClicked = async () => {
  // Submit to server.
  var submissionData = new FormData();

  submissionData.append("file", submissionContent);

  // axios.post("/submission_input_save", submissionData, {
  //   headers: {
  //     "file-name": "test",
  //     "track": "B",
  //     "description": submissionDescription.value,
  //     "Access-Control-Allow-Origin": "*",
  //   }
  // });

  let requestData = {
    user_id: await userID
  }
  axios.post("/new_submission_track_b", requestData, {
    headers: {
      "content-type": "application/json",
      "Access-Control-Allow-Origin": "*",
    }
  }).then((response) => {
    axios.post("/submission_input_save", submissionData, {
      headers: {
        "file-name": response.data,
        "track": "B",
        "description": submissionDescription.value,
        "Access-Control-Allow-Origin": "*",
      }
    })
  });

  processed = true;
  await forceRerender();

  console.log("Submission sent to server!");
}
</script>
