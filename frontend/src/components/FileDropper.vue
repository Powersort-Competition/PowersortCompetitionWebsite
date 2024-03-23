<template>
  <div class="main">
    <div class="dropzone-container" @dragover="dragover" @dragleave="dragleave" @drop="drop">
      <input
        type="file"
        multiple
        name="file"
        id="fileInput"
        class="hidden-input"
        @change="onChange"
        ref="file"
        accept=".txt"
        />

      <label for="fileInput" class="file-label">
        <div v-if="isDragging">Release to drop submission file here!</div>
        <div v-if="hasDropped">Processing...</div>
        <div v-else>Drop submission text (txt) file here!</div>
      </label>
      </div>
  </div>
</template>

<script>
export default
{
  data() {
    return {
      isDragging: false,
      hasDropped: false,
      file: []
    };
  },
  methods: {
    onChange() {
      this.file.push(...this.$refs.file.files);

      const reader = new FileReader();
      reader.onload = (res) =>
      {
        this.submission_content = res.target.result;
      };
      reader.readAsText(this.file[0]);

      this.hasDropped = true;

      this.$emit('file-dropped', this.submission_content);
    },
    dragover(e) {
      e.preventDefault();
      this.isDragging = true;
    },
    dragleave() {
      this.isDragging = false;
    },
    drop(e) {
      e.preventDefault();
      this.$refs.file.files = e.dataTransfer.files;
      this.onChange();
      this.isDragging = false;
      this.hasDropped = true;
    },
  },
};
</script>
<style scoped src="../filedropper.css"></style>