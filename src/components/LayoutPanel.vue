<template>
  <div class="layout-panel">
    <div
      class="layout-panel-header"
      :class="{ 'layout-panel-header-collapsed': props.collapsed }"
    >
      <div
        class="layout-panel-header-title"
        :class="{ 'layout-panel-header-title-collapsed': props.collapsed }"
      >
        <!-- TODO: Maybe add header text here? -->
      </div>
      <div class="layout-panel-header-actions">
        <slot name="header-actions" />
      </div>
    </div>
    <div class="layout-panel-content" :class="{ collapsed: props.collapsed }">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  header?: string;
  collapsed: boolean;
  toggleable: boolean;
  /**
   * When the panel is collapsed, if the toggle icon/header should be displayed horizontally or vertically.
   * This is only relevant when toggleable is true.
   */
  collapseOrientation: "horizontal" | "vertical";
}>();

const emit = defineEmits<(e: "update:collapsed", collapsed: boolean) => void>();
</script>

<style scoped>
.layout-panel {
  display: flex;
  flex-direction: column;

  border: 1px solid var(--p-panel-header-border-color);
  border-radius: 0.5rem;

  max-height: 100vh;
  max-width: 100vw;

  height: 100%;
  width: 100%;
}

.layout-panel-content {
  max-height: 100%;
  max-width: 100%;
}

.layout-panel-header {
  display: flex;
  justify-content: space-between;
}

.layout-panel-header:has(.layout-panel-header-title-text) {
  padding: 1rem 1.5rem;
}

.layout-panel-content:has(
    ~ .layout-panel-header .layout-panel-header-title-text
  ) {
  padding: 0 1.5rem 1.5rem;
}

.layout-panel-header-collapsed {
  flex-direction: column-reverse;
  align-items: flex-end;
  gap: 0.5rem;
}

.layout-panel-header-title-collapsed {
  writing-mode: vertical-rl;
}

.layout-panel-header-title {
  font-weight: bold;
}

.layout-panel-content.collapsed {
  display: none;
}
</style>
