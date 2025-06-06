<template>

	<div class="relative w-full h-full pointer-events-none">

		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 512 512"
			class="absolute inset-0 pointer-events-none z-10 opacity-80"
		>

			<defs>

				<marker
					id="arrowhead"
					markerWidth="10"
					markerHeight="7"
					refX="3"
					refY="2"
					orient="auto"
					markerUnits="strokeWidth"
				>

					<polygon
						points="0 0, 3 2, 0 4"
						:fill="computedColor"
					/>

				</marker>

			</defs>

			<rect
				width="512"
				height="512"
				fill="none"
			/>

			<line
				:x1="from.x"
				:y1="from.y"
				:x2="to.x"
				:y2="to.y"
				fill="none"
				:stroke="computedColor"
				stroke-linecap="round"
				stroke-linejoin="round"
				:stroke-width="computedSize"
				marker-end="url(#arrowhead)"
			/>

		</svg>

	</div>

</template>

<script setup lang="ts">
import { computed, toRefs } from "vue";

const props = defineProps<{
	to: {
		x: number;
		y: number;
	};
	from: {
		x: number;
		y: number;
	};
	options: {
		color?: string;
		size?: number;
	};
}>();

const { color, size } = toRefs(props.options);

const computedSize = computed(() => {
	return size?.value ?? 10;
});

const computedColor = computed(() => {
	return color?.value ?? "currentColor";
});
</script>

