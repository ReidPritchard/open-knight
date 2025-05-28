<template>
  <Teleport to="body">
    <div
      v-if="visible"
      ref="menuRef"
      class="fixed z-50 menu bg-base-100 rounded-box shadow-lg border border-base-300 p-1 min-w-48"
      :style="menuStyle"
      @click.stop
      @contextmenu.prevent
    >
      <template v-for="(item, index) in items" :key="item.id || index">
        <!-- Divider -->
        <li v-if="item.type === 'divider'" class="divider h-0.5 my-0.5"></li>

        <!-- Menu Item -->
        <li v-else>
          <a
            @click="handleItemClick(item)"
            class="flex items-center gap-2"
            :class="{
              'text-error hover:bg-error hover:text-error-content':
                item.type === 'destructive',
              'opacity-50 cursor-not-allowed': item.disabled,
            }"
            :tabindex="item.disabled ? -1 : 0"
            @keydown.enter="handleItemClick(item)"
            @keydown.space.prevent="handleItemClick(item)"
          >
            <component v-if="item.icon" :is="item.icon" class="w-4 h-4" />
            {{ item.label }}
          </a>
        </li>
      </template>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import type { MenuItem } from "./types";

interface Props {
	visible: boolean;
	x: number;
	y: number;
	items: MenuItem[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
	"item-click": [itemId: string];
	close: [];
}>();

const menuRef = ref<HTMLElement | null>(null);

// Calculate menu position with edge detection
const menuStyle = computed(() => {
	if (!props.visible) return {};

	let { x, y } = props;

	// Get viewport dimensions
	const viewportWidth = window.innerWidth;
	const viewportHeight = window.innerHeight;

	// Estimate menu dimensions (will be refined after mount)
	const menuWidth = 192; // min-w-48 = 12rem = 192px
	const menuHeight = props.items.length * 40; // Rough estimate

	// Adjust x position if menu would overflow right edge
	if (x + menuWidth > viewportWidth) {
		x = Math.max(8, viewportWidth - menuWidth - 8);
	}

	// Adjust y position if menu would overflow bottom edge
	if (y + menuHeight > viewportHeight) {
		y = Math.max(8, viewportHeight - menuHeight - 8);
	}

	// Ensure minimum distance from edges
	x = Math.max(8, x);
	y = Math.max(8, y);

	return {
		left: `${x}px`,
		top: `${y}px`,
	};
});

// Handle menu item clicks
const handleItemClick = (item: MenuItem) => {
	if (item.disabled || item.type === "divider") return;

	emit("item-click", item.id);
	emit("close");
};

// Handle click outside to close menu
const handleClickOutside = (event: MouseEvent) => {
	if (!props.visible) return;

	const target = event.target as Node;
	if (menuRef.value && !menuRef.value.contains(target)) {
		emit("close");
	}
};

// Handle escape key to close menu
const handleKeyDown = (event: KeyboardEvent) => {
	if (!props.visible) return;

	if (event.key === "Escape") {
		event.preventDefault();
		emit("close");
	}
};

// Refine positioning after menu is mounted
const refinePosition = async () => {
	if (!props.visible || !menuRef.value) return;

	await nextTick();

	const rect = menuRef.value.getBoundingClientRect();
	const viewportWidth = window.innerWidth;
	const viewportHeight = window.innerHeight;

	let { x, y } = props;
	let needsUpdate = false;

	// Check if menu overflows and adjust
	if (rect.right > viewportWidth) {
		x = Math.max(8, viewportWidth - rect.width - 8);
		needsUpdate = true;
	}

	if (rect.bottom > viewportHeight) {
		y = Math.max(8, viewportHeight - rect.height - 8);
		needsUpdate = true;
	}

	if (needsUpdate && menuRef.value) {
		menuRef.value.style.left = `${x}px`;
		menuRef.value.style.top = `${y}px`;
	}
};

// Watch for visibility changes to refine positioning
watch(
	() => props.visible,
	(visible) => {
		if (visible) {
			nextTick(() => refinePosition());
		}
	},
);

onMounted(() => {
	document.addEventListener("click", handleClickOutside);
	document.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
	document.removeEventListener("click", handleClickOutside);
	document.removeEventListener("keydown", handleKeyDown);
});
</script>

<style scoped>
/* Ensure menu appears above other elements */
.menu {
  box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
}

/* Focus styles for accessibility */
.menu li a:focus {
  outline: 2px solid hsl(var(--p));
  outline-offset: -2px;
}

/* Animation for menu appearance */
.menu {
  animation: menuFadeIn 0.15s ease-out;
}

@keyframes menuFadeIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
