<template>

	<div class="overflow-x-auto">

		<ul class="menu menu-sm w-full">

			<template
				v-for="(row, index) in tableRows"
				:key="index"
			>

				<li v-if="row.type === 'move'">

					<div class="flex flex-row justify-evenly w-full">

						<MoveButton
							v-if="row.white"
							:move-data="row.white"
							:is-current="isCurrentMove(row.white.nodeId)"
							@click="handleMoveClick"
							size="xs"
						/>

						<MoveButton
							v-if="row.black"
							:move-data="row.black"
							:is-current="isCurrentMove(row.black.nodeId)"
							@click="handleMoveClick"
							size="xs"
						/>

					</div>

				</li>

				<li v-else-if="row.type === 'variation' && showVariations">

					<VariationRow
						:moves="row.moves"
						:is-current-move="isCurrentMove"
						@move-click="handleMoveClick"
					/>

				</li>

			</template>

		</ul>

	</div>

</template>

<script setup lang="ts">
import MoveButton from "../MoveButton/MoveButton.vue";
import VariationRow from "../VariationRow/VariationRow.vue";
import type { TableRow, NodeId } from "../../../shared/types";

defineProps<{
	tableRows: TableRow[];
	showVariations: boolean;
	isCurrentMove: (nodeId: NodeId) => boolean;
}>();

const emit = defineEmits<{
	"move-select": [moveId: number | undefined];
}>();

const handleMoveClick = (moveId: number | undefined) => {
	emit("move-select", moveId);
};
</script>

