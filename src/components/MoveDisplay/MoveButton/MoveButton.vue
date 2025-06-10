<template>

	<button
		@click="handleClick"
		:class="{
			'btn btn-ghost': true,
			'btn-accent btn-soft': props.isCurrent,
			'btn-neutral': !props.isCurrent,
			'btn-sm': props.size === 'xs',
			'btn-md': props.size === 'sm',
		}"
	>

		<span
			v-if="props.moveData.showNumber"
			class="font-bold"
			:class="{
				'text-accent': props.isCurrent,
				'text-neutral-content/80': !props.isCurrent,
			}"
		>
			 {{ moveNumber }}
		</span>

		<span
			class="font-mono"
			:class="{
				'text-accent': props.isCurrent,
				'text-neutral-content': !props.isCurrent,
			}"
		>
			 {{ props.moveData.san }}
		</span>

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

