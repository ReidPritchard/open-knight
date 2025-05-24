<template>
  <div class="relative flex" :class="containerClasses" :style="containerStyle">
    <!-- Panel Content -->
    <div class="flex-1 overflow-hidden">
      <slot />
    </div>

    <!-- Resize Handle -->
    <div
      v-if="!isCollapsed"
      :class="handleClasses"
      @mousedown="startResize"
      @dblclick="toggleCollapse"
    >
      <div :class="handleBarClasses"></div>
    </div>

    <!-- Collapse/Expand Button -->
    <button
      v-if="collapsible"
      :class="collapseButtonClasses"
      @click="toggleCollapse"
      :title="isCollapsed ? 'Expand panel' : 'Collapse panel'"
    >
      <component :is="collapseIcon" class="w-3 h-3" />
    </button>
  </div>
</template>

<script setup lang="ts">
import {
  PhCaretDown,
  PhCaretLeft,
  PhCaretRight,
  PhCaretUp,
} from "@phosphor-icons/vue";
import { computed, onMounted, onUnmounted, ref } from "vue";

interface Props {
  initialSize: number;
  minSize?: number;
  maxSize?: number;
  direction: "horizontal" | "vertical";
  position?: "left" | "right" | "top" | "bottom";
  collapsible?: boolean;
  resizable?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  minSize: 100,
  maxSize: 1000,
  position: "left",
  collapsible: true,
  resizable: true,
});

const emit = defineEmits<{
  resize: [size: number];
  collapse: [collapsed: boolean];
}>();

const currentSize = ref(props.initialSize);
const isCollapsed = ref(false);
const isResizing = ref(false);

// Computed properties for styling
const containerClasses = computed(() => ({
  "flex-row": props.direction === "horizontal",
  "flex-col": props.direction === "vertical",
}));

const containerStyle = computed(() => {
  if (isCollapsed.value) {
    return props.direction === "horizontal"
      ? { width: "32px" }
      : { height: "32px" };
  }

  return props.direction === "horizontal"
    ? { width: `${currentSize.value}px` }
    : { height: `${currentSize.value}px` };
});

const handleClasses = computed(() => ({
  "absolute cursor-col-resize hover:bg-primary/20 transition-colors":
    props.direction === "horizontal",
  "absolute cursor-row-resize hover:bg-primary/20 transition-colors":
    props.direction === "vertical",
  "right-0 top-0 w-1 h-full":
    props.direction === "horizontal" && props.position === "left",
  "left-0 top-0 w-1 h-full":
    props.direction === "horizontal" && props.position === "right",
  "bottom-0 left-0 w-full h-1":
    props.direction === "vertical" && props.position === "top",
  "top-0 left-0 w-full h-1":
    props.direction === "vertical" && props.position === "bottom",
  "bg-primary/50": isResizing.value,
}));

const handleBarClasses = computed(() => ({
  "w-full h-8 bg-base-300 opacity-0 hover:opacity-100 transition-opacity rounded-sm":
    props.direction === "horizontal",
  "h-full w-8 bg-base-300 opacity-0 hover:opacity-100 transition-opacity rounded-sm":
    props.direction === "vertical",
  "absolute top-1/2 -translate-y-1/2": props.direction === "horizontal",
  "absolute left-1/2 -translate-x-1/2": props.direction === "vertical",
}));

const collapseButtonClasses = computed(() => ({
  "absolute z-10 btn btn-xs btn-ghost btn-soft btn-neutral": true,
  "right-1 top-2":
    props.direction === "horizontal" && props.position === "left",
  "left-1 top-2":
    props.direction === "horizontal" && props.position === "right",
  "top-1 right-2":
    props.direction === "vertical" && props.position === "bottom",
  "bottom-1 right-2":
    props.direction === "vertical" && props.position === "top",
}));

const collapseIcon = computed(() => {
  if (props.direction === "horizontal") {
    if (isCollapsed.value) {
      return props.position === "left" ? PhCaretRight : PhCaretLeft;
    }
    return props.position === "left" ? PhCaretLeft : PhCaretRight;
  }
  if (isCollapsed.value) {
    return props.position === "top" ? PhCaretDown : PhCaretUp;
  }
  return props.position === "top" ? PhCaretUp : PhCaretDown;
});

// Resize functionality
const startResize = (event: MouseEvent) => {
  if (!props.resizable) return;

  event.preventDefault();
  isResizing.value = true;

  const startX = event.clientX;
  const startY = event.clientY;
  const startSize = currentSize.value;

  const handleMouseMove = (e: MouseEvent) => {
    let delta = 0;

    if (props.direction === "horizontal") {
      delta =
        props.position === "left" ? e.clientX - startX : startX - e.clientX;
    } else {
      delta =
        props.position === "top" ? e.clientY - startY : startY - e.clientY;
    }

    const newSize = Math.max(
      props.minSize,
      Math.min(props.maxSize, startSize + delta)
    );

    currentSize.value = newSize;
    emit("resize", newSize);
  };

  const handleMouseUp = () => {
    isResizing.value = false;
    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  };

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
};

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value;
  emit("collapse", isCollapsed.value);
};

// Keyboard shortcuts
const handleKeyPress = (event: KeyboardEvent) => {
  if (event.ctrlKey && event.key === "[") {
    toggleCollapse();
  }
};

onMounted(() => {
  document.addEventListener("keydown", handleKeyPress);
});

onUnmounted(() => {
  document.removeEventListener("keydown", handleKeyPress);
});
</script>

<style scoped>
/* Custom resize cursor for better UX */
.cursor-col-resize:hover::after {
  content: "";
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 2px;
  height: 20px;
  background: currentColor;
  opacity: 0.5;
}

.cursor-row-resize:hover::after {
  content: "";
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 20px;
  height: 2px;
  background: currentColor;
  opacity: 0.5;
}
</style>
