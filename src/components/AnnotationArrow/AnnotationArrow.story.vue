<template>

	<Story
		title="Annotation Arrow"
		:layout="{ type: 'grid' }"
	>

		<template #controls>

			<div class="flex flex-col gap-2">

				<div class="flex flex-row gap-2">

					<HstNumber
						v-model="state.startX"
						title="Start X"
					/>

					<HstNumber
						v-model="state.startY"
						title="Start Y"
					/>

				</div>

				<div class="flex flex-row gap-2">

					<HstNumber
						v-model="state.endX"
						title="End X"
					/>

					<HstNumber
						v-model="state.endY"
						title="End Y"
					/>

				</div>

				<div class="flex flex-row gap-2">

					<HstSelect
						v-model="state.color"
						:options="usedColors"
						title="Color"
					/>

					<HstNumber
						v-model="state.size"
						title="Size"
					/>

				</div>

			</div>

		</template>

		<Variant title="Default">

			<AnnotationArrow
				:from="{ x: state.startX, y: state.startY }"
				:to="{ x: state.endX, y: state.endY }"
				:options="{ color: state.color, size: state.size }"
			/>

		</Variant>

	</Story>

</template>

<script setup lang="ts">
import { reactive } from "vue";
import AnnotationArrow from "./AnnotationArrow.vue";

const usedColors = ["blue", "green", "red", "yellow"];

const state = reactive({
	startX: 0,
	startY: 0,
	endX: 128,
	endY: 128,
	color: usedColors[0],
	size: 5,
});
</script>

