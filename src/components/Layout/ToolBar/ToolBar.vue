<template>

	<ul
		ref="toolbarRef"
		class="menu menu-horizontal menu-sm bg-base-100 rounded-box w-full justify-around flex-nowrap"
	>

		<li
			v-for="(item, index) in displayedItems"
			:key="index"
			@click="item.action ? item.action() : null"
			class="mx-2 transition-all duration-200"
		>

			<slot
				name="toolbar-item"
				:icon-component="iconMap[item.icon as keyof typeof iconMap]"
				v-bind="item"
			>

				<div
					class="tooltip tooltip-bottom b-0 p-0 z-20"
					:data-tip="item.label"
				>

					<button
						class="btn btn-sm"
						@click="item.action"
					>

						<component
							v-if="item.icon && iconMap[item.icon as keyof typeof iconMap]"
							:is="iconMap[item.icon as keyof typeof iconMap]"
						/>

						<span
							v-if="showLabels"
							class="label duration-200 ml-2"
						>
							 {{ item.label }}
						</span>

					</button>

				</div>

			</slot>

		</li>

	</ul>

</template>

<script lang="ts" setup>
import { computed, ref, onMounted, onUnmounted, nextTick, watch } from "vue";
import {
	PhBinary,
	PhIdentificationCard,
	PhPlay,
	PhRepeat,
	PhStop,
	PhDetective,
	PhWaveformSlash,
	PhBackspace,
	PhFilePlus,
} from "@phosphor-icons/vue";

const props = defineProps<{
	toolbarItems: {
		label: string;
		icon?: string;
		action?: () => void;
		hidden?: boolean;
	}[];
}>();

const toolbarRef = ref<HTMLElement>();
const showLabels = ref(true);
let resizeObserver: ResizeObserver | null = null;

const displayedItems = computed(() => {
	return props.toolbarItems.filter((item) => !item.hidden);
});

const iconMap = {
	PhBinary,
	PhIdentificationCard,
	PhPlay,
	PhRepeat,
	PhStop,
	PhDetective,
	PhWaveformSlash,
	PhBackspace,
	PhFilePlus,
};

const checkOverflow = async () => {
	if (!toolbarRef.value) return;

	// First, try showing labels
	showLabels.value = true;

	// Wait for DOM update
	await nextTick();

	// Check if content overflows
	const hasOverflow =
		toolbarRef.value.scrollWidth > toolbarRef.value.clientWidth;

	if (hasOverflow) {
		showLabels.value = false;
	}
};

onMounted(() => {
	if (toolbarRef.value) {
		// Initial check
		checkOverflow();

		// Watch for resize changes
		resizeObserver = new ResizeObserver(() => {
			checkOverflow();
		});

		resizeObserver.observe(toolbarRef.value);
	}
});

onUnmounted(() => {
	if (resizeObserver) {
		resizeObserver.disconnect();
	}
});

// Also watch for changes in displayedItems
watch(displayedItems, () => {
	nextTick(() => {
		checkOverflow();
	});
});
</script>

