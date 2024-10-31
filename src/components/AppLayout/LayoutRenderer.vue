<template>
  <component :is="layoutComponent" :layout="layout" />
</template>

<script lang="ts">
import { defineComponent, computed, Component } from "vue";
import { ILayout } from "../../shared/types";
import WindowRenderer from "./WindowRenderer.vue";
import ContainerRenderer from "./ContainerRenderer.vue";

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
