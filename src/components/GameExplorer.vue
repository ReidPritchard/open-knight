<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const san = ref("");

async function sanToMove() {
  moveResult.value = await invoke("san_to_move", { san: san.value });
}

const moveResult = ref("");
</script>

<template>
  <form class="row" @submit.prevent="sanToMove">
    <input id="san-input" v-model="san" placeholder="Enter SAN..." />
    <button type="submit">Convert</button>
  </form>

  <p>Move: {{ moveResult }}</p>
</template>
