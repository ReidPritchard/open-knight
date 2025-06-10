<template>

	<div class="overflow-x-auto">

		<table class="table table-xs">

			<tbody>

				<template
					v-for="(row, index) in tableRows"
					:key="index"
				>

					<tr v-if="row.type === 'move'">

						<td
							colspan="9"
							class="w-1/2"
						>

							<MoveButton
								v-if="row.white"
								:move-data="row.white"
								:is-current="isCurrentMove(row.white.nodeId)"
								class="w-full"
								@click="handleMoveClick"
							/>

						</td>

						<td
							colspan="9"
							class="w-1/2"
						>

							<MoveButton
								v-if="row.black"
								:move-data="row.black"
								:is-current="isCurrentMove(row.black.nodeId)"
								class="w-full"
								@click="handleMoveClick"
							/>

						</td>

					</tr>

					<!-- Variation row(s) (18 columns) -->

					<VariationRow
						v-else-if="row.type === 'variation' && showVariations"
						:moves="row.moves"
						:is-current-move="isCurrentMove"
						:max-depth="10"
						:row-size="18"
						@move-click="handleMoveClick"
					/>

				</template>

			</tbody>

		</table>

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

