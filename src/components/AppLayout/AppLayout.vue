<script setup lang="ts">
import { defineProps, onMounted, onUnmounted } from "vue";
import { ILayout } from "../../shared/types";
import WindowItem from "./WindowItem.vue";

const props = defineProps<{
  layout: ILayout;
}>();
const emit = defineEmits(["update:layout", "update:toggle-collapse"]);

const handleKeyboard = (e: KeyboardEvent) => {
  if (e.ctrlKey && e.key === 'b') {
    emit('update:toggle-collapse', 'side-bar');
  }
  if (e.ctrlKey && e.key === '\'') {
    emit('update:toggle-collapse', 'right-sidebar');
  }
  if (e.ctrlKey && e.key === 'j') {
    emit('update:toggle-collapse', 'bottom-panel');
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyboard);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyboard);
});
</script>

<template>
  <div class="app-layout">
    <WindowItem :window="props.layout" @update:toggle-collapse="emit('update:toggle-collapse', $event)"
      @update:layout="emit('update:layout', $event)">
      <!-- Forward all slots to WindowItem -->
      <template v-for="(_, name) in $slots" :key="name" #[name]>
        <slot :name="name" />
      </template>
    </WindowItem>
  </div>
</template>

<style scoped>
.app-layout {
  position: relative;

  padding: 0;
  margin: 0;

  width: 100%;
  height: 100%;
}
</style>
