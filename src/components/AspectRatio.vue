<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  ratio?: string;
}>();

const aspectRatio = computed(() => props.ratio || "16 / 9");

const padding = computed(() => {
  const [width, height] = aspectRatio.value.split(" / ").map(Number);
  return `${(height / width) * 100}%`;
});
</script>

<template>
  <div
    class="aspect-ratio-box"
    :style="{
      aspectRatio: props.ratio || '16 / 9',
      '--padding': padding,
    }"
  >
    <div class="content-wrapper">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.aspect-ratio-box {
  position: relative;
  overflow: hidden;
  max-height: 94%;
  max-width: 100%;
  padding-bottom: var(--padding);
}

.content-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
