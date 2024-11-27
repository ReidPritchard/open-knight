<template>
  <div class="flex flex-col gap-6 p-4">
    <!-- PGN Text Input -->
    <div class="flex flex-col gap-2">
      <label
        for="pgn-text"
        class="text-sm font-medium text-gray-700 dark:text-gray-200"
      >
        PGN Text Input
      </label>
      <textarea
        id="pgn-text"
        v-model="pgnText"
        rows="16"
        placeholder="Paste your PGN text here..."
        class="w-full px-3 py-2 text-sm text-gray-700 dark:text-gray-200 placeholder-gray-400 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors resize-y"
      />
    </div>

    <!-- File Upload -->
    <div class="flex flex-col gap-2">
      <label
        for="pgn-file"
        class="text-sm font-medium text-gray-700 dark:text-gray-200"
      >
        File Upload
      </label>
      <div class="flex flex-col items-center justify-center w-full">
        <label
          class="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 dark:border-gray-600 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors"
        >
          <div class="flex flex-col items-center justify-center pt-5 pb-6">
            <svg
              class="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 20 16"
            >
              <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2"
              />
            </svg>
            <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
              <span class="font-semibold">Click to upload</span> or drag and
              drop
            </p>
            <p class="text-xs text-gray-500 dark:text-gray-400">
              PGN files (max. 10MB)
            </p>
          </div>
          <input
            id="pgn-file"
            type="file"
            class="hidden"
            accept=".pgn"
            @change="handleFileUpload"
          />
        </label>
        <!-- File name display -->
        <div
          v-if="fileName"
          class="mt-2 text-sm text-gray-600 dark:text-gray-300"
        >
          Selected file: {{ fileName }}
        </div>
      </div>
    </div>

    <!-- Import Button -->
    <button
      @click="closeDialog"
      class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
    >
      Import
    </button>
  </div>
</template>

<script setup lang="ts">
import { type Ref, inject, ref } from "vue";

interface DialogRef {
  close: () => void;
}

const pgnText = ref("");
const pgnFile = ref<File | null>(null);
const fileName = ref<string>("");

const dialogRef = inject<Ref<DialogRef>>("dialogRef");

const handleFileUpload = (event: Event) => {
  const input = event.target as HTMLInputElement;
  if (input.files && input.files.length > 0) {
    const file = input.files[0];
    if (file.size > 10000000) {
      // TODO: Add proper error handling
      alert("File size exceeds 10MB limit");
      return;
    }
    pgnFile.value = file;
    fileName.value = file.name;
  }
};

const closeDialog = () => {
  dialogRef?.value?.close();
};
</script>
