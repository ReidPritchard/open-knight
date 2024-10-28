<script setup lang="ts">
import { defineProps } from "vue";
import { ILayout } from "../../shared/types";
import WindowItem from "./WindowItem.vue";

const props = defineProps<{
  layout: ILayout;
}>();
const emit = defineEmits(["update:layout", "update:toggle-collapse"]);
</script>

<template>
  <div class="app-layout">
    <WindowItem
      :window="props.layout"
      @update:toggle-collapse="emit('update:toggle-collapse', $event)"
      @update:layout="emit('update:layout', $event)"
    >
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
