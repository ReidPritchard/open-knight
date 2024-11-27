<template>
  <div
    class="transition-all duration-300 ease-in-out relative"
    :class="[orientation === 'vertical' ? 'w-full' : 'h-full']"
    :style="style"
  >
    <div class="h-full">
      <div
        class="flex h-full border rounded-lg bg-white dark:bg-gray-800 shadow-sm overflow-hidden"
        :class="{ 'items-center justify-center': collapsed }"
      >
        <!-- Collapsed State -->
        <button
          v-if="collapsed"
          @click="$emit('toggle')"
          class="p-2 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
        >
          <slot name="collapse-icon" />
        </button>
        <!-- Expanded State -->
        <div v-else class="w-full overflow-hidden flex flex-col">
          <!-- Header -->
          <div
            class="flex items-center justify-between p-3 border-b border-gray-200 dark:border-gray-700 shrink-0"
          >
            <template v-if="orientation === 'vertical'">
              <span class="text-sm font-medium">{{ title }}</span>
              <button
                @click="$emit('toggle')"
                class="p-1 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
              >
                <slot name="expand-icon" />
              </button>
            </template>
            <template v-else>
              <span class="text-sm font-medium">{{ title }}</span>
              <button
                @click="$emit('toggle')"
                class="p-1 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
              >
                <slot name="expand-icon" />
              </button>
            </template>
          </div>
          <!-- Content -->
          <div class="p-1 overflow-auto">
            <slot />
          </div>
        </div>
      </div>
    </div>
    <!-- Resize Handle -->
    <div
      v-if="!collapsed && resizable"
      :class="[
        orientation === 'vertical'
          ? 'absolute right-0 top-0 bottom-0 w-1 cursor-col-resize'
          : 'absolute top-0 left-0 right-0 h-1 cursor-row-resize',
        'hover:bg-blue-500/50 active:bg-blue-500',
      ]"
      @mousedown="$emit('resize-start', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  title: string;
  collapsed: boolean;
  orientation: "vertical" | "horizontal";
  resizable?: boolean;
  size?: number;
  collapsedSize?: number;
}>();

defineEmits<{
  (e: "toggle"): void;
  (e: "resize-start", event: MouseEvent): void;
}>();

const style = computed(() => {
  const dimension = props.orientation === "vertical" ? "width" : "height";
  return {
    [dimension]: props.collapsed
      ? `${props.collapsedSize ?? 48}px`
      : `${props.size}px`,
  };
});
</script>
