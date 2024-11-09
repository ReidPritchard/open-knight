<template>
  <div :style="windowStyle">
    <Panel
      v-if="layout.collapsible"
      :header="layout.title || layout.id"
      :collapsed="layout.collapsed"
      toggleable
      @toggle="onToggle"
    >
      <component :is="contentComponent" v-if="contentComponent" />
    </Panel>
    <component v-else :is="contentComponent" v-if="contentComponent" />
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from "vue";
import { IWindow } from "../../shared/types";
import Panel from "primevue/panel";
import componentRegistry from "../../componentRegistry";

export default defineComponent({
  name: "WindowRenderer",
  components: {
    Panel,
  },
  props: {
    layout: {
      type: Object as () => IWindow,
      required: true,
    },
  },
  setup(props) {
    const windowStyle = computed(() => {
      const style: any = {
        display: "flex",
        flexDirection: "column",
      };
      if (props.layout.fixedSize) {
        style.flexBasis = `${props.layout.fixedSize}px`;
        style.flexGrow = 0;
      } else {
        style.flexGrow = props.layout.size || 1;
      }
      return style;
    });

    const contentComponent = computed(() => {
      console.log(props.layout.contentComponent);
      return componentRegistry[props.layout.contentComponent] || null;
    });

    const onToggle = (event: any) => {
      props.layout.collapsed = event.value;
    };

    return {
      windowStyle,
      contentComponent,
      onToggle,
    };
  },
});
</script>

<style scoped>
/* Optional styles */
</style>
