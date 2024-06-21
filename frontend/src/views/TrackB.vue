<template>
  <FileDropper @file-dropped="handleFileDrop"/>
</template>

<script setup>
import FileDropper from "@/components/FileDropper.vue";
import axios from "axios";

const handleFileDrop = async(submission_content) => {
  console.log("File dropped!");

}
</script>