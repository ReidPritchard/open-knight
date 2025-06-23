<template>

	<div class="relative w-full h-full pointer-events-none">

		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 512 512"
			width="512"
			height="512"
			class="absolute inset-0 pointer-events-none z-10"
		>

			<defs>

				<filter
					id="arrowShadow"
					x="-50%"
					y="-50%"
					width="200%"
					height="200%"
				>

					<feDropShadow
						dx="1"
						dy="1"
						stdDeviation="1.5"
						flood-opacity="0.3"
					/>

				</filter>

				<marker
					:id="`arrowhead-${markerId}`"
					markerWidth="12"
					markerHeight="10"
					refX="6"
					refY="5"
					orient="auto"
					markerUnits="strokeWidth"
				>

					<polygon
						points="0,1 8,5 0,9 2,5"
						:fill="computedColor"
						stroke="none"
					/>

				</marker>

			</defs>

			<line
				:x1="adjustedFrom.x"
				:y1="adjustedFrom.y"
				:x2="adjustedTo.x"
				:y2="adjustedTo.y"
				fill="none"
				stroke="rgba(0, 0, 0, 0.2)"
				stroke-linecap="round"
				:stroke-width="computedSize + 2"
			/>

			<line
				:x1="adjustedFrom.x"
				:y1="adjustedFrom.y"
				:x2="adjustedTo.x"
				:y2="adjustedTo.y"
				fill="none"
				:stroke="computedColor"
				stroke-linecap="round"
				stroke-linejoin="round"
				:stroke-width="computedSize"
				:marker-end="`url(#arrowhead-${markerId})`"
				filter="url(#arrowShadow)"
				opacity="0.9"
			/>

		</svg>

	</div>

</template>

<script setup lang="ts">
import { computed } from "vue";

interface ArrowOptions {
	color?: string;
	size?: number;
}

interface Position {
	x: number;
	y: number;
}

interface Props {
	from: Position;
	to: Position;
	options?: ArrowOptions;
}

const props = withDefaults(defineProps<Props>(), {
	options: () => ({}),
});

const computedSize = computed((): number => {
	return props.options.size ?? 8;
});

const computedColor = computed((): string => {
	return props.options.color ?? "#fbbf24";
});

// Generate unique marker ID to avoid conflicts when multiple arrows have different colors
const markerId = computed((): string => {
	return computedColor.value.replace("#", "");
});

// Adjust arrow positioning to account for arrowhead size
const adjustedFrom = computed((): Position => {
	const dx = props.to.x - props.from.x;
	const dy = props.to.y - props.from.y;
	const length = Math.sqrt(dx * dx + dy * dy);

	if (length === 0) return props.from;

	const unitX = dx / length;
	const unitY = dy / length;
	const offset = computedSize.value * 0.3;

	return {
		x: props.from.x + unitX * offset,
		y: props.from.y + unitY * offset,
	};
});

const adjustedTo = computed((): Position => {
	const dx = props.to.x - props.from.x;
	const dy = props.to.y - props.from.y;
	const length = Math.sqrt(dx * dx + dy * dy);

	if (length === 0) return props.to;

	const unitX = dx / length;
	const unitY = dy / length;
	// Increase offset to prevent line from showing through arrowhead
	const offset = computedSize.value * 1.2;

	return {
		x: props.to.x - unitX * offset,
		y: props.to.y - unitY * offset,
	};
});
</script>

