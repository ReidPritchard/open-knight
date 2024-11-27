<template>
  <WindowRenderer
    v-if="isWindowComponent(props.layout)"
    :layout="props.layout"
  />
  <ContainerRenderer v-else :layout="props.layout" />
</template>

<script setup lang="ts">
import type { ILayout, IWindow } from "../../shared/types";
import ContainerRenderer from "./ContainerRenderer.vue";
import WindowRenderer from "./WindowRenderer.vue";

const props = defineProps<{
  layout: ILayout;
}>();

// Type guard for window components
function isWindowComponent(layout: ILayout): layout is IWindow {
  return "contentComponent" in layout;
}
</script>

<style scoped>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}
</style>
