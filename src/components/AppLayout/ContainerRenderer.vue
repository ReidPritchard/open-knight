<template>
  <LayoutPanel
    :header="props.layout.title"
    :toggleable="props.layout.collapsible"
    :collapsed="props.layout.collapsed"
    :collapse-orientation="'vertical'"
    @update:collapsed="props.layout.collapsed = $event"
  >
    <div :style="containerStyle" class="content">
      <div
        v-if="props.layout.display === 'flexible'"
        :class="['flex-container', orientationClass]"
      >
        <template v-for="child in props.layout.children" :key="child.id">
          <LayoutRenderer :layout="child" />
        </template>
      </div>

      <div v-else-if="props.layout.display === 'tabs'">
        <Tabs :value="activeTabIndex">
          <TabList>
            <Tab
              v-for="child in props.layout.children"
              :header="child.title || child.id"
              :key="child.id"
              :value="child.id"
            >
              {{ child.title || child.id }}
            </Tab>
          </TabList>
          <TabPanels>
            <TabPanel
              v-for="child in props.layout.children"
              :header="child.title || child.id"
              :key="child.id"
              :value="child.id"
            >
              <LayoutRenderer :layout="child" />
            </TabPanel>
          </TabPanels>
        </Tabs>
      </div>

      <div
        v-else-if="props.layout.display === 'simple'"
        class="simple-container"
      >
        <div v-if="props.layout.top" class="top-panel">
          <LayoutRenderer :layout="props.layout.top" />
        </div>
        <div class="middle-content">
          <div v-if="props.layout.left" class="left-panel">
            <LayoutRenderer :layout="props.layout.left" />
          </div>
          <div class="center-content">
            <template v-for="child in props.layout.children" :key="child.id">
              <LayoutRenderer :layout="child" />
            </template>
          </div>
          <div v-if="props.layout.right" class="right-panel">
            <LayoutRenderer :layout="props.layout.right" />
          </div>
        </div>
        <div v-if="props.layout.bottom" class="bottom-panel">
          <LayoutRenderer :layout="props.layout.bottom" />
        </div>
      </div>
    </div>
  </LayoutPanel>
</template>

<script setup lang="ts">
import Tab from "primevue/tab";
import TabList from "primevue/tablist";
import TabPanel from "primevue/tabpanel";
import TabPanels from "primevue/tabpanels";
import Tabs from "primevue/tabs";
import { computed } from "vue";
import {
  type IWindowContainer,
  validateFlexibleContainer,
  validateTabContainer,
} from "../../shared/types";
import LayoutPanel from "../LayoutPanel.vue";
import LayoutRenderer from "./LayoutRenderer.vue";

const props = defineProps<{
  layout: IWindowContainer;
}>();

const orientationClass = computed(() => {
  const flexibleContainer = validateFlexibleContainer(props.layout);
  if (flexibleContainer.success) {
    return flexibleContainer.data.orientation === "vertical"
      ? "flex-column"
      : "flex-row";
  }
  return "";
});

const containerStyle = computed(() => {
  const style: Record<string, string | number> = {
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

const activeTabIndex = computed(() => {
  return (
    props.layout.children.findIndex((child) => {
      const tabContainer = validateTabContainer(child);
      return tabContainer.success && tabContainer.data.activeTabId;
    }) || 0
  );
});
</script>

<style scoped>
* {
  --p-panel-content-padding: 0rem;
  --p-panel-toggleable-header-padding: 0.5rem;

  max-width: 100%;
  max-height: 100%;
}

.content {
  display: flex;
  flex-direction: column;

  /* border: 1px solid red; */

  max-width: 100%;
  max-height: 100%;

  height: 100%;
  width: 100%;
}

.flex-container {
  display: flex;

  max-height: 100%;
  max-width: 100%;

  height: 100%;
  width: 100%;
}

.flex-row {
  flex-direction: row;
}

.flex-column {
  flex-direction: column;
}

.simple-container {
  display: flex;
  flex-direction: column;

  max-width: 100%;
  max-height: 100%;
}

.top-panel,
.bottom-panel {
  /* flex-shrink: 0; */

  max-width: 100%;
}

.middle-content {
  display: flex;
}

.left-panel,
.right-panel {
  /* flex-shrink: 0; */
  max-width: 50%;
  max-height: 100%;
}

.center-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}
</style>
