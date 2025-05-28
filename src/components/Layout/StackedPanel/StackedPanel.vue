<template>
  <div class="flex flex-col h-full overflow-hidden bg-base-300">
    <!-- Tab Mode -->
    <div v-if="mode === 'tabs'" class="flex flex-col flex-1 min-h-0">
      <!-- DaisyUI Tabs -->
      <div class="tabs tabs-bordered overflow-hidden flex-1">
        <template v-for="section in sections" :key="section.id">
          <input
            type="radio"
            :name="`${props.name}-tab`"
            class="tab"
            :aria-label="section.title"
            @keydown="handleKeyDown"
            @change="setActiveTab(section.id)"
            :checked="activeTabId === section.id"
            :class="{
              'tab-active': activeTabId === section.id,
              'tab-disabled': section.disabled,
            }"
          />
          <div
            class="tab-content border-base-300 bg-base-100 min-h-0 overflow-auto hidden"
            :class="{
              // Display flex if the section is active
              'flex flex-col': activeTabId === section.id,
            }"
          >
            <slot :name="section.id" :section="section" />
          </div>
        </template>
      </div>
    </div>

    <!-- Accordion Mode -->
    <div
      v-else-if="mode === 'accordion'"
      class="flex-1 min-h-0 flex flex-col overflow-auto"
    >
      <slot />
    </div>

    <!-- Vertical Mode -->
    <div v-else class="flex-1 min-h-0 flex flex-col overflow-auto">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { provide, ref, watch } from "vue";
import type { Component } from "vue";

export interface StackedPanelSection {
  id: string;
  title: string;
  icon?: Component;
  badge?: string | number;
  badgeClass?: string;
  disabled?: boolean;
  visible?: boolean;
}

interface Props {
  name: string;
  mode?: "tabs" | "accordion" | "vertical";
  sections?: StackedPanelSection[];
  activeTab?: string;
  size?: "sm" | "md" | "lg";
  variant?: "default" | "primary" | "secondary";
  persistState?: boolean;
  storageKey?: string;
}

const props = withDefaults(defineProps<Props>(), {
  mode: "accordion",
  sections: () => [],
  size: "md",
  variant: "default",
  persistState: false,
  storageKey: "stacked-panel-state",
});

const emit = defineEmits<{
  tabChange: [tabId: string];
  sectionToggle: [sectionId: string, collapsed: boolean];
}>();

// State
const activeTabId = ref(props.activeTab || props.sections[0]?.id || "");
// Methods
const setActiveTab = (tabId: string) => {
  if (activeTabId.value === tabId) return;

  activeTabId.value = tabId;
  emit("tabChange", tabId);

  if (props.persistState) {
    saveState();
  }
};

const saveState = () => {
  if (!props.persistState) return;

  const state = {
    activeTab: activeTabId.value,
    timestamp: Date.now(),
  };

  localStorage.setItem(props.storageKey, JSON.stringify(state));
};

const loadState = () => {
  if (!props.persistState) return;

  try {
    const saved = localStorage.getItem(props.storageKey);
    if (!saved) return;

    const state = JSON.parse(saved);
    if (
      state.activeTab &&
      props.sections.some((s) => s.id === state.activeTab)
    ) {
      activeTabId.value = state.activeTab;
    }
  } catch (error) {
    console.warn("Failed to load stacked panel state:", error);
  }
};

// Provide context for child sections
provide("stackedPanel", {
  mode: props.mode,
  size: props.size,
  variant: props.variant,
  onSectionToggle: (sectionId: string, collapsed: boolean) => {
    emit("sectionToggle", sectionId, collapsed);
  },
});

// Watchers
watch(
  () => props.activeTab,
  (newTab) => {
    if (newTab && newTab !== activeTabId.value) {
      activeTabId.value = newTab;
    }
  }
);

watch(
  () => props.sections,
  (newSections) => {
    // Ensure active tab is valid
    if (!newSections.some((s) => s.id === activeTabId.value)) {
      const firstVisible = newSections.find((s) => s.visible !== false);
      if (firstVisible) {
        activeTabId.value = firstVisible.id;
      }
    }
  },
  { immediate: true }
);

// Lifecycle
loadState();

// Keyboard navigation for tabs
const handleKeyDown = (event: KeyboardEvent) => {
  if (props.mode !== "tabs") return;

  const currentIndex = props.sections.findIndex(
    (s) => s.id === activeTabId.value
  );
  let newIndex = currentIndex;

  switch (event.key) {
    case "ArrowLeft":
      newIndex = Math.max(0, currentIndex - 1);
      break;
    case "ArrowRight":
      newIndex = Math.min(props.sections.length - 1, currentIndex + 1);
      break;
    case "Home":
      newIndex = 0;
      break;
    case "End":
      newIndex = props.sections.length - 1;
      break;
    default:
      return;
  }

  if (newIndex !== currentIndex) {
    event.preventDefault();
    const newSection = props.sections[newIndex];
    if (!newSection.disabled) {
      setActiveTab(newSection.id);
    }
  }
};

// Expose methods
defineExpose({
  setActiveTab,
  getActiveTab: () => activeTabId.value,
  saveState,
  loadState,
});
</script>
