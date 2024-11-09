<template>
  <div class="pgn-import">
    <div class="pgn-text-import">
      <label for="pgn-text">PGN Text Input</label>
      <Textarea
        v-model="pgnText"
        rows="16"
        cols="60"
        auto-resize
        placeholder="Paste your PGN text here..."
      />
    </div>
    <div class="pgn-file-import">
      <label for="pgn-file">File Upload</label>
      <FileUpload v-model="pgnFile" :max-file-size="10000000" />
    </div>

    <Button label="Import" @click="closeDialog" />
  </div>
</template>

<script setup lang="ts">
import Textarea from "primevue/textarea";
import FileUpload from "primevue/fileupload";
import { ref, inject } from "vue";

const pgnText = ref("");
const pgnFile = ref<File | null>(null);

const dialogRef = inject("dialogRef");

const closeDialog = () => {
  (dialogRef as any).value.close();
};
</script>

<style scoped>
.pgn-import {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.pgn-text-import {
  padding: 1rem;

  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.pgn-file-import {
  padding: 1rem;

  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
</style>
