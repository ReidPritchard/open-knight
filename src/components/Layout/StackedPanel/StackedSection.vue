<template>

	<div
		class="border-b border-base-200 last:border-b-0"
		:class="sectionSizeClasses"
	>

		<!-- Section Header -->

		<div
			class="flex items-center justify-between p-3 focus:outline-none focus:ring-2 focus:ring-primary/50 transition-colors"
			:class="[
				headerVariantClasses,
				headerSizeClasses,
				collapsible ? 'cursor-pointer hover:bg-base-100' : '',
				headerClass,
			]"
			@click="toggleCollapse"
			:tabindex="collapsible ? 0 : -1"
			@keydown.enter="toggleCollapse"
			@keydown.space.prevent="toggleCollapse"
		>

			<div class="flex items-center gap-2 flex-1 min-w-0">

				<!-- Icon -->

				<component
					v-if="icon"
					:is="icon"
					class="w-4 h-4 flex-shrink-0"
					:class="iconClasses"
				/>

				<!-- Title -->

				<h3
					class="font-medium truncate"
					:class="titleClasses"
				>
					 {{ title }}
				</h3>

				<!-- Badge -->

				<div
					v-if="badge"
					class="badge badge-xs"
					:class="badgeClasses"
				>
					 {{ badge }}
				</div>

			</div>

			<!-- Header Actions -->

			<div class="flex items-center gap-1">

				<slot name="header-actions" />

				<!-- Collapse Toggle -->

				<button
					v-if="collapsible"
					class="btn btn-ghost btn-xs p-1"
					@click.stop="toggleCollapse"
					:title="isCollapsed ? 'Expand section' : 'Collapse section'"
				>

					<component
						:is="collapseIcon"
						class="w-3 h-3 transition-transform duration-200"
						:class="{ 'rotate-180': isCollapsed }"
					/>

				</button>

			</div>

		</div>

		<!-- Section Content -->

		<Transition
			name="section-collapse"
			@enter="onEnter"
			@after-enter="onAfterEnter"
			@leave="onLeave"
			@after-leave="onAfterLeave"
		>

			<div
				v-show="!isCollapsed"
				class="overflow-hidden"
				:class="contentClass"
			>

				<div
					class="flex-1 min-h-0"
					:class="[textSizeClasses]"
					:style="contentStyle"
				>

					<slot />

				</div>

			</div>

		</Transition>

	</div>

</template>

<script setup lang="ts">
import { PhCaretDown } from "@phosphor-icons/vue";
import { computed, ref } from "vue";
import type { Component } from "vue";

interface Props {
	title: string;
	icon?: Component;
	badge?: string | number;
	collapsible?: boolean;
	collapsed?: boolean;
	variant?: "default" | "primary" | "secondary" | "accent";
	size?: "sm" | "md" | "lg";
	headerClass?: string;
	contentClass?: string;
	minHeight?: number;
	maxHeight?: number;
}

const props = withDefaults(defineProps<Props>(), {
	collapsible: true,
	collapsed: false,
	variant: "default",
	size: "md",
	minHeight: 0,
});

const emit = defineEmits<{
	toggle: [collapsed: boolean];
	expand: [];
	collapse: [];
}>();

const isCollapsed = ref(props.collapsed);

// Computed classes using Tailwind/DaisyUI
const sectionSizeClasses = computed(() => ({
	"stacked-section--sm": props.size === "sm",
	"stacked-section--md": props.size === "md",
	"stacked-section--lg": props.size === "lg",
}));

const headerVariantClasses = computed(() => ({
	"bg-primary/5 text-primary": props.variant === "primary",
	"bg-secondary/5 text-secondary": props.variant === "secondary",
	"bg-accent/5 text-accent": props.variant === "accent",
}));

const headerSizeClasses = computed(() => ({
	"py-2": props.size === "sm",
	"py-3": props.size === "md",
	"py-4": props.size === "lg",
}));

const titleClasses = computed(() => ({
	"text-xs": props.size === "sm",
	"text-sm": props.size === "md",
	"text-base": props.size === "lg",
}));

const textSizeClasses = computed(() => ({
	"text-xs": props.size === "sm",
	"text-sm": props.size === "md",
	"text-base": props.size === "lg",
}));

const iconClasses = computed(() => ({
	"text-primary": props.variant === "primary",
	"text-secondary": props.variant === "secondary",
	"text-accent": props.variant === "accent",
	"text-base-content/70": props.variant === "default",
}));

const badgeClasses = computed(() => ({
	"badge-primary": props.variant === "primary",
	"badge-secondary": props.variant === "secondary",
	"badge-accent": props.variant === "accent",
	"badge-neutral": props.variant === "default",
}));

const contentStyle = computed(() => ({
	minHeight: props.minHeight ? `${props.minHeight}px` : undefined,
}));

const collapseIcon = computed(() => PhCaretDown);

// Methods
const toggleCollapse = () => {
	if (!props.collapsible) return;

	isCollapsed.value = !isCollapsed.value;
	emit("toggle", isCollapsed.value);

	if (isCollapsed.value) {
		emit("collapse");
	} else {
		emit("expand");
	}
};

// Animation handlers
const onEnter = (el: Element) => {
	const element = el as HTMLElement;
	element.style.height = "0";
	element.offsetHeight; // Force reflow
	element.style.height = `${element.scrollHeight}px`;
};

const onAfterEnter = (el: Element) => {
	const element = el as HTMLElement;
	element.style.height = "auto";
};

const onLeave = (el: Element) => {
	const element = el as HTMLElement;
	element.style.height = `${element.scrollHeight}px`;
	element.offsetHeight; // Force reflow
	element.style.height = "0";
};

const onAfterLeave = (el: Element) => {
	const element = el as HTMLElement;
	element.style.height = "auto";
};

// Expose methods for parent component
defineExpose({
	toggle: toggleCollapse,
	expand: () => {
		if (isCollapsed.value) toggleCollapse();
	},
	collapse: () => {
		if (!isCollapsed.value) toggleCollapse();
	},
	isCollapsed: () => isCollapsed.value,
});
</script>

<style scoped>
.section-collapse-enter-active,
.section-collapse-leave-active {
  transition: height 0.3s ease;
  overflow: hidden;
}

.section-collapse-enter-from,
.section-collapse-leave-to {
  height: 0;
}
</style>

