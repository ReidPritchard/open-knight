<template>
  <component :is="layoutComponent" :layout="layout" />
</template>

<script lang="ts">
import { type Component, computed, defineComponent } from "vue";
import type { ILayout } from "../../shared/types";
import ContainerRenderer from "./ContainerRenderer.vue";
import WindowRenderer from "./WindowRenderer.vue";

export default defineComponent({
  name: "LayoutRenderer",
  components: {
    WindowRenderer,
    ContainerRenderer,
  } satisfies Record<string, Component>,
  props: {
    layout: {
      type: Object as () => ILayout,
      required: true,
    },
  },
  setup(props) {
    const layoutComponent = computed(() => {
      return "contentComponent" in props.layout
        ? "WindowRenderer"
        : "ContainerRenderer";
    });

    return {
      layoutComponent,
    };
  },
});
</script>

<style scoped>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}
</style>
