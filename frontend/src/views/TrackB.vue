<template>
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
            max-rows="6"
        />

        <BButton @click="buttonClicked">Submit</BButton>
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
</template>

<script setup>
import FileDropper from "@/components/FileDropper.vue";
import axios from "axios";
import {nextTick, ref} from "vue";
import {BFormTextarea, BButton, BAlert} from "bootstrap-vue-next";

const fileDropComponent = ref(true);
const submissionDescription = ref();

let processed = false;
let submissionContent = null;

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

  axios.post("/submission_input_save", submissionData, {
    headers: {
      "file-name": "test",
      "track": "B",
      "description": submissionDescription.value,
      "Access-Control-Allow-Origin": "*",
    }
  });

  processed = true;
  await forceRerender();

  console.log("Submission sent to server!");
}
</script>