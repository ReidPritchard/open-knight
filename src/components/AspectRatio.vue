<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  ratio: number;
}>();

const outerStyle = computed(() => ({
  width: "100%",
}));

const innerStyle = computed(() => ({
  paddingTop: `${props.ratio * 100}%`,
}));
</script>

<template>
  <!-- Outer container -->
  <div class="outer-container" :style="outerStyle">
    <!-- Inner container -->
    <div class="inner-container" :style="innerStyle">
      <!-- Content -->
      <div class="content">
        <slot />
      </div>
    </div>
  </div>
</template>

<style scoped>
.inner-container {
  position: relative;
}

.content {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
</style>
