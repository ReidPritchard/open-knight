<template>
  <div class="inplace-edit">
    <div v-if="!editing" class="display-content" @click="startEdit">
      <slot name="display"></slot>
    </div>
    <div v-else class="edit-content">
      <slot name="content" :closeCallback="finishEdit"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const editing = ref(false);

const startEdit = () => {
  editing.value = true;
};

const finishEdit = () => {
  editing.value = false;
};
</script>

<style scoped>
.inplace-edit {
  width: 100%;
}

.display-content,
.edit-content {
  width: 100%;
}

.display-content {
  cursor: pointer;
}
</style>
