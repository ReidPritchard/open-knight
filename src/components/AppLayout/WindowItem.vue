<template>
  <component :is="wrapperComponent" v-bind="wrapperProps">
    <section class="app-layout__window" :id="window.id" :aria-label="`${window.id} window`"
      :aria-expanded="!isCollapsed" role="region" tabindex="0"
      @keydown.space.prevent="emit('update:toggle-collapse', window.id)">
      <div class="app-layout__window-content" :class="containerClasses"
        :style="{ display: isCollapsed ? 'none' : 'flex' }">
        <template v-if="hasChildren">
          <component :is="containerComponent" v-bind="containerProps">
            <component v-for="childWindow in (window as IWindowContainer).children" :key="childWindow.id"
              :is="itemComponent" :window="childWindow" :header="childWindow.id">
              <SlotForwarder />
            </component>
          </component>
        </template>
        <template v-else>
          <slot :name="window.id" />
        </template>
      </div>
      <ContextMenu :model="contextMenuItems" :target="`.app-layout__window#${window.id}`" />
    </section>
  </component>
</template>

<script setup lang="ts">
import Accordion from 'primevue/accordion';
import AccordionTab from 'primevue/accordiontab';
import Panel from 'primevue/panel';
import TabPanel from 'primevue/tabpanel';
import TabView from 'primevue/tabview';
import ContextMenu from 'primevue/contextmenu';
import { computed } from 'vue';
import SlotForwarder from './SlotForwarder.vue';

import {
  ILayout,
  IWindowContainer,
  validateWindowContainer,
  WindowDisplay,
  PanelPosition,
} from '../../shared/types';

const props = defineProps<{ window: ILayout }>();
const emit = defineEmits([
  'update:window',
  'update:toggle-collapse',
  'close-window',
]);

const isContainer = computed(() => validateWindowContainer(props.window));
const hasChildren = computed(
  () =>
    isContainer.value.success &&
    (props.window as IWindowContainer).children.length > 0
);
const isCollapsed = computed(
  () =>
    isContainer.value.success && (props.window as IWindowContainer).collapsed
);

const wrapperProps = computed(() =>
  props.window.id !== 'root' ? { header: props.window.id, toggleable: true } : {}
);

const wrapperComponent = computed(() =>
  props.window.id !== 'root' && isContainer.value.success ? Panel : 'div'
);

const displayMode = computed(() =>
  isContainer.value.success
    ? (props.window as IWindowContainer).display
    : null
);

const containerComponent = computed(() => {
  switch (displayMode.value) {
    case WindowDisplay.Tabs:
      return TabView;
    case WindowDisplay.Accordion:
      return Accordion;
    case WindowDisplay.Panel:
      return 'div';
    default:
      return 'div';
  }
});

const itemComponent = computed(() => {
  switch (displayMode.value) {
    case WindowDisplay.Tabs:
      return TabPanel;
    case WindowDisplay.Accordion:
      return AccordionTab;
    default:
      return 'WindowItem';
  }
});

const containerProps = computed(() => {
  if (displayMode.value === WindowDisplay.Panel) {
    return { class: 'panel-container', style: panelStyles.value };
  }
  return {};
});

const containerClasses = computed(() => ({
  container: isContainer.value.success,
  horizontal:
    isContainer.value.success &&
    (props.window as IWindowContainer).direction === 'horizontal',
  vertical:
    isContainer.value.success &&
    (props.window as IWindowContainer).direction === 'vertical',
}));

const panelStyles = computed(() => {
  if (
    !isContainer.value.success ||
    (props.window as IWindowContainer).display !== WindowDisplay.Panel
  ) {
    return {};
  }

  const container = props.window as IWindowContainer;
  const position = container.panelPosition || PanelPosition.Left;
  const isCollapsed = container.collapsed;
  const size = isCollapsed ? '32px' : `${container.size}px`;

  const positions = {
    [PanelPosition.Left]: { left: 0, width: size },
    [PanelPosition.Right]: { right: 0, width: size },
    [PanelPosition.Top]: { top: 0, height: size },
    [PanelPosition.Bottom]: { bottom: 0, height: size },
  };

  return {
    position: 'absolute',
    top: positions[position].top || 0,
    bottom: positions[position].bottom || 0,
    left: positions[position].left || 0,
    right: positions[position].right || 0,
    width: positions[position].width || 'auto',
    height: positions[position].height || 'auto',
    transition: 'all 0.2s ease',
  };
});

const contextMenuItems = computed(() => [
  {
    label: props.window.collapsed ? 'Expand' : 'Collapse',
    icon: props.window.collapsed
      ? 'pi pi-window-maximize'
      : 'pi pi-window-minimize',
    command: () => emit('update:toggle-collapse', props.window.id),
  },
  {
    label: 'Close',
    icon: 'pi pi-times',
    command: () => emit('close-window', props.window.id),
  },
]);
</script>

<style scoped>
* {
  user-select: none;
}

.window-item {
  width: 100%;
  height: 100%;
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

.panel-container {
  background-color: var(--p-card-background);
  border: 1px solid var(--p-border-color);
  z-index: 10;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px;
  cursor: pointer;
  background-color: var(--p-surface-hover);
}

.panel-header:hover {
  background-color: var(--p-surface-hover);
}

.panel-title {
  font-weight: bold;
}

.panel-content {
  height: calc(100% - 36px);
  overflow: auto;
}

.rotate-180 {
  transform: rotate(180deg);
}
</style>
