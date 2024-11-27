<template>
  <div :style="windowStyle">
    <div
      v-if="layout.collapsible"
      class="border rounded-lg bg-white dark:bg-gray-800 shadow-sm"
    >
      <button
        @click="onToggle({ value: !layout.collapsed })"
        class="w-full px-4 py-3 flex items-center justify-between text-left border-b bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors rounded-t-lg"
      >
        <span class="font-medium">{{ layout.title || layout.id }}</span>
        <svg
          class="w-5 h-5 transform transition-transform"
          :class="{ 'rotate-180': !layout.collapsed }"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
            clip-rule="evenodd"
          />
        </svg>
      </button>
      <div v-show="!layout.collapsed" class="p-4">
        <component :is="contentComponent" v-if="contentComponent" />
      </div>
    </div>
    <component v-else :is="contentComponent" v-if="contentComponent" />
  </div>
</template>

<script lang="ts">
import { type CSSProperties, computed, defineComponent } from "vue";
import componentRegistry from "../../componentRegistry";
import type { IWindow } from "../../shared/types";

interface ToggleEvent {
  value: boolean;
}

export default defineComponent({
  name: "WindowRenderer",
  props: {
    layout: {
      type: Object as () => IWindow,
      required: true,
    },
  },
  setup(props) {
    const windowStyle = computed(() => {
      const style: CSSProperties = {
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

    const onToggle = (event: ToggleEvent) => {
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
