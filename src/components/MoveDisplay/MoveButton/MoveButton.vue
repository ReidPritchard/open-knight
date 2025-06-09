<template>

	<button
		@click="handleClick"
		class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer flex items-center gap-2"
		:class="buttonClass"
	>

		<span
			v-if="props.moveData.showNumber"
			class="font-bold"
			:class="props.isCurrent ? 'text-primary-content' : 'text-base-content/60'"
		>
			 {{ moveNumber }}
		</span>

		<span class="font-mono"> {{ props.moveData.san }} </span>

	</button>

</template>

<script setup lang="ts">
import { computed } from "vue";
import type { MoveData } from "../../../shared/types";

const props = withDefaults(
	defineProps<{
		moveData: MoveData;
		isCurrent?: boolean;
		variant?: "main" | "variation";
		size?: "sm" | "xs";
	}>(),
	{
		isCurrent: false,
		variant: "main",
		size: "sm",
	},
);

const emit = defineEmits<{
	click: [moveId: number | undefined];
}>();

const buttonClass = computed(() => {
	const baseClasses =
		props.size === "xs" ? "px-1.5 py-0.5 text-xs" : "px-2 py-1 text-sm";

	if (props.isCurrent) {
		if (props.variant === "variation") {
			return `${baseClasses} bg-secondary text-secondary-content border border-secondary`;
		}
		return `${baseClasses} bg-primary text-primary-content`;
	}

	if (props.variant === "variation") {
		return `${baseClasses} bg-base-100 border border-base-300 hover:bg-base-300`;
	}

	return `${baseClasses} hover:bg-base-300`;
});

const moveNumber = computed(() => {
	// Display "2." and "2.." for white and black moves respectively
	if (props.moveData.isWhite) {
		return `${props.moveData.moveNumber}.`;
	}
	return `${props.moveData.moveNumber}..`;
});

const handleClick = () => {
	emit("click", props.moveData.move?.id);
};
</script>

