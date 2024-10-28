<template>
  <component :is="wrapperComponent" v-bind="wrapperProps">
    <section
      class="app-layout__window"
      :id="window.id"
      :aria-labelledby="`window-title-${window.id}`"
      role="dialog"
    >
      <div
        class="app-layout__window-content"
        :class="containerClasses"
        :style="{ display: isCollapsed ? 'none' : 'flex' }"
      >
        <template v-if="hasChildren">
          <!-- Render different components based on display mode -->
          <template v-if="displayMode === WindowDisplay.Tabs">
            <TabView>
              <TabPanel
                v-for="childWindow in (window as IWindowContainer).children"
                :key="childWindow.id"
                :header="childWindow.id"
              >
                <WindowItem :window="childWindow">
                  <!-- Forward all slots to child WindowItems -->
                  <template v-for="(_, name) in $slots" :key="name" #[name]>
                    <slot :name="name" />
                  </template>
                </WindowItem>
              </TabPanel>
            </TabView>
          </template>
          <template v-else-if="displayMode === WindowDisplay.Accordion">
            <Accordion>
              <AccordionTab
                v-for="childWindow in (window as IWindowContainer).children"
                :key="childWindow.id"
                :header="childWindow.id"
              >
                <WindowItem :window="childWindow">
                  <!-- Forward all slots to child WindowItems -->
                  <template v-for="(_, name) in $slots" :key="name" #[name]>
                    <slot :name="name" />
                  </template>
                </WindowItem>
              </AccordionTab>
            </Accordion>
          </template>
          <template v-else-if="displayMode === WindowDisplay.Split">
            <Splitter :layout="splitterLayout">
              <SplitterPanel
                v-for="childWindow in (window as IWindowContainer).children"
                :key="childWindow.id"
              >
                <WindowItem :window="childWindow">
                  <!-- Forward all slots to child WindowItems -->
                  <template v-for="(_, name) in $slots" :key="name" #[name]>
                    <slot :name="name" />
                  </template>
                </WindowItem>
              </SplitterPanel>
            </Splitter>
          </template>
          <!-- Default rendering -->
          <template v-else>
            <WindowItem
              v-for="childWindow in (window as IWindowContainer).children"
              :key="childWindow.id"
              :window="childWindow"
            >
              <!-- Forward all slots to child WindowItems -->
              <template v-for="(_, name) in $slots" :key="name" #[name]>
                <slot :name="name" />
              </template>
            </WindowItem>
          </template>
        </template>
        <!-- If this is a leaf window, render its named slot -->
        <template v-else>
          <slot :name="window.id" />
        </template>
      </div>
    </section>
  </component>
</template>

<script setup lang="ts">
import Accordion from "primevue/accordion";
import AccordionTab from "primevue/accordiontab";
import Panel from "primevue/panel";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import TabPanel from "primevue/tabpanel";
import TabView from "primevue/tabview";
import { computed } from "vue";
import WindowItem from "./WindowItem.vue";

import {
  ILayout,
  IWindowContainer,
  validateWindowContainer,
  WindowDisplay,
} from "../../shared/types";

const props = defineProps<{
  window: ILayout;
}>();

const wrapperProps = computed(() => {
  return props.window.id !== "root"
    ? { header: props.window.id, toggleable: true }
    : {};
});

const isContainer = computed(() => validateWindowContainer(props.window));

const wrapperComponent = computed(() => {
  return props.window.id !== "root" && isContainer.value.success
    ? Panel
    : "div";
});

const hasChildren = computed(
  () =>
    isContainer.value.success &&
    (props.window as IWindowContainer).children.length > 0
);

const isCollapsed = computed(
  () =>
    isContainer.value.success && (props.window as IWindowContainer).collapsed
);

const displayMode = computed(() => {
  if (isContainer.value.success) {
    const display = (props.window as IWindowContainer).display;
    switch (display) {
      case WindowDisplay.Tabs:
        return "tabs";
      case WindowDisplay.Split:
        return "split";
      case WindowDisplay.Accordion:
        return "accordion";
      default:
        return null;
    }
  }
  return null;
});

const splitterLayout = computed(() => {
  if (isContainer.value.success) {
    const direction = (props.window as IWindowContainer).direction;
    return direction === "horizontal" ? "horizontal" : "vertical";
  }
  return "horizontal"; // default
});

const containerClasses = computed(() => {
  return {
    container: isContainer.value.success,
    horizontal:
      isContainer.value.success &&
      (props.window as IWindowContainer).direction === "horizontal",
    vertical:
      isContainer.value.success &&
      (props.window as IWindowContainer).direction === "vertical",
  };
});
</script>

<style scoped>
* {
  user-select: none;
}

.app-layout__window {
  width: 100%;
  height: 100%;
  color: var(--p-card-color);
  background-color: var(--p-card-background);
  border: 1px solid var(--p-border-color);
  overflow: hidden;
}

.app-layout__window-content {
  width: 100%;
  height: 100%;
  padding: 8px;
  user-select: none;
}

.app-layout__window-content.container {
  display: flex;
  justify-content: center;
  align-items: stretch;
}

.app-layout__window-content.container.horizontal {
  flex-direction: row;
}

.app-layout__window-content.container.vertical {
  flex-direction: column;
}

/* Additional styles for PrimeVue components if needed */
</style>
