<template>
  <div
    :style="containerStyle"
    class="flex flex-col w-full h-full max-w-full max-h-full overflow-hidden"
    :data-container-id="layout.id"
  >
    <!-- Flexible Layout -->
    <div
      v-if="layout.display === WindowDisplayMode.Flexible"
      :class="[
        'flex w-full h-full max-w-full max-h-full overflow-hidden items-stretch justify-start',
        layout.orientation === LayoutDirection.Vertical
          ? 'flex-col'
          : 'flex-row',
      ]"
    >
      <template v-for="child in layout.children" :key="child.id">
        <LayoutRenderer :layout="child" />
      </template>
    </div>

    <!-- Tabs Layout -->
    <div
      v-else-if="layout.display === WindowDisplayMode.Tabs"
      class="w-full h-full overflow-hidden"
    >
      <div
        class="flex flex-col h-full bg-white dark:bg-gray-800 rounded-lg shadow-sm"
      >
        <!-- Tab Headers -->
        <div
          class="flex border-b border-gray-200 dark:border-gray-700 shrink-0"
        >
          <button
            v-for="(child, index) in layout.children"
            :key="child.id"
            :class="[
              'px-4 py-2 text-sm font-medium transition-colors',
              index === activeTabIndex
                ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
                : 'text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200',
            ]"
            @click="selectTab(index)"
          >
            {{ child.title || child.id }}
          </button>
        </div>
        <!-- Tab Panels -->
        <div class="flex-1 p-4 overflow-auto">
          <div
            v-for="(child, index) in layout.children"
            :key="child.id"
            v-show="index === activeTabIndex"
            class="h-full"
          >
            <LayoutRenderer :layout="child" />
          </div>
        </div>
      </div>
    </div>

    <!-- Simple Layout -->
    <div
      v-else-if="layout.display === WindowDisplayMode.Simple"
      class="flex flex-col w-full h-full overflow-hidden"
    >
      <!-- Top Panel -->
      <Panel
        v-if="simpleContainer?.top"
        :title="simpleContainer.top.title || 'Top Panel'"
        :collapsed="simpleContainer.top.collapsed"
        orientation="horizontal"
        :resizable="true"
        :size="topPanelHeight"
        :collapsedSize="48"
        @toggle="togglePanel('top')"
        @resize-start="startResize('top', $event)"
      >
        <template #collapse-icon>
          <svg
            class="w-6 h-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M5 15l7-7 7 7"
            />
          </svg>
        </template>
        <template #expand-icon>
          <svg
            class="w-5 h-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </template>
        <LayoutRenderer :layout="simpleContainer.top" />
      </Panel>

      <!-- Middle Content -->
      <div class="flex flex-1 min-h-0 overflow-hidden">
        <!-- Left Panel -->
        <Panel
          v-if="simpleContainer?.left"
          :title="simpleContainer.left.title || 'Left Panel'"
          :collapsed="simpleContainer.left.collapsed"
          orientation="vertical"
          :resizable="true"
          :size="leftPanelWidth"
          :collapsedSize="48"
          @toggle="togglePanel('left')"
          @resize-start="startResize('left', $event)"
        >
          <template #collapse-icon>
            <svg
              class="w-6 h-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 5l7 7-7 7"
              />
            </svg>
          </template>
          <template #expand-icon>
            <svg
              class="w-5 h-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </template>
          <LayoutRenderer :layout="simpleContainer.left" />
        </Panel>

        <!-- Center Content -->
        <div class="flex flex-col flex-1 min-w-0 px-4 overflow-hidden">
          <template v-for="child in layout.children" :key="child.id">
            <LayoutRenderer :layout="child" />
          </template>
        </div>

        <!-- Right Panel -->
        <Panel
          v-if="simpleContainer?.right"
          :title="simpleContainer.right.title || 'Right Panel'"
          :collapsed="simpleContainer.right.collapsed"
          orientation="vertical"
          :resizable="true"
          :size="rightPanelWidth"
          :collapsedSize="48"
          @toggle="togglePanel('right')"
          @resize-start="startResize('right', $event)"
        >
          <template #collapse-icon>
            <svg
              class="w-6 h-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 5l-7 7 7 7"
              />
            </svg>
          </template>
          <template #expand-icon>
            <svg
              class="w-5 h-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 19l7-7-7-7"
              />
            </svg>
          </template>
          <LayoutRenderer :layout="simpleContainer.right" />
        </Panel>
      </div>

      <!-- Bottom Panel -->
      <Panel
        v-if="simpleContainer?.bottom"
        :title="simpleContainer.bottom.title || 'Bottom Panel'"
        :collapsed="simpleContainer.bottom.collapsed"
        orientation="horizontal"
        :resizable="true"
        :size="bottomPanelHeight"
        :collapsedSize="48"
        @toggle="togglePanel('bottom')"
        @resize-start="startResize('bottom', $event)"
      >
        <template #collapse-icon>
          <svg
            class="w-5 h-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M5 15l7-7 7 7"
            />
          </svg>
        </template>
        <template #expand-icon>
          <svg
            class="w-6 h-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </template>
        <LayoutRenderer :layout="simpleContainer.bottom" />
      </Panel>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useResizePanel } from "../../composables/useResizePanel";
import type { ISimpleContainer, IWindowContainer } from "../../shared/types";
import { LayoutDirection, WindowDisplayMode } from "../../shared/types";
import LayoutRenderer from "./LayoutRenderer.vue";
import Panel from "./Panel.vue";

const props = defineProps<{
  layout: IWindowContainer;
}>();

const simpleContainer = computed(() => {
  return props.layout.display === WindowDisplayMode.Simple
    ? (props.layout as ISimpleContainer)
    : null;
});

const containerStyle = computed(() => {
  const style: Record<string, string> = {};
  if (props.layout.fixedSize) {
    style.flexBasis = `${props.layout.fixedSize}px`;
    style.flexGrow = "0";
  } else {
    style.flexGrow = props.layout.size?.toString() || "1";
  }
  return style;
});

// Tab handling
const activeTabIndex = ref(0);
const selectTab = (index: number) => {
  activeTabIndex.value = index;
};

// Panel resizing
const {
  leftPanelWidth,
  rightPanelWidth,
  topPanelHeight,
  bottomPanelHeight,
  startResize,
} = useResizePanel(
  {
    leftPanelWidth: 250,
    rightPanelWidth: 250,
    topPanelHeight: 250,
    bottomPanelHeight: 250,
  },
  {
    MIN_PANEL_WIDTH: 200,
    MAX_PANEL_WIDTH: 600,
    MIN_PANEL_HEIGHT: 100,
    MAX_PANEL_HEIGHT: 500,
  }
);

// Panel toggling
const togglePanel = (panel: "left" | "right" | "top" | "bottom") => {
  if (!simpleContainer.value || !simpleContainer.value[panel]) return;
  const targetPanel = simpleContainer.value[panel];
  if (targetPanel) {
    targetPanel.collapsed = !targetPanel.collapsed;
  }
};
</script>
