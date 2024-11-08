<script setup lang="ts">
const props = defineProps<{
  ratio?: number | string;
}>();
</script>

<template>
  <div
    class="aspect-ratio-box"
    :style="{ aspectRatio: props.ratio || '16 / 9' }"
  >
    <div class="content-wrapper">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.aspect-ratio-box {
  width: 100%;
  position: relative;
  overflow: hidden;
}

.content-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
</style>
