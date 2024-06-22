<template>
  <div v-if="fileDropComponent">
    <div v-if="processed === false">
      <FileDropper @file-dropped="handleFileDrop"/>
    </div>
    <div v-else-if="processed === true">
      Processed!
    </div>
  </div>
</template>

<script setup>
import FileDropper from "@/components/FileDropper.vue";
import axios from "axios";
import {nextTick, ref} from "vue";

const fileDropComponent = ref(true);
let processed = false;

const forceRerender = async () => {
  fileDropComponent.value = false;

  await nextTick();

  fileDropComponent.value = true;
}

const handleFileDrop = async (submission_content) => {
  console.log("File dropped!");

  var submission_input_data = new FormData();
  submission_input_data.append("file", submission_content);

  // Upload the file to the server.
  axios.post("/submission_input_save", submission_input_data, {
    headers: {
      "file-name": "test",
      "track": "B",
      "Access-Control-Allow-Origin": "*",
    }
  });

  processed = true;
  await forceRerender();
}
</script>