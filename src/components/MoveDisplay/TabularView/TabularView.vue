<template>

	<div class="overflow-x-auto">

		<table class="table table-xs">

			<tbody>

				<template
					v-for="(row, index) in tableRows"
					:key="index"
				>

					<!-- Main move row (9 + 9 = 18 columns) -->

					<tr v-if="row.type === 'move'">

						<td colspan="9">

							<MoveButton
								v-if="row.white"
								:move-data="row.white"
								:is-current="isCurrentMove(row.white.nodeId)"
								class="w-full text-left"
								@click="handleMoveClick"
							/>

						</td>

						<td colspan="9">

							<MoveButton
								v-if="row.black"
								:move-data="row.black"
								:is-current="isCurrentMove(row.black.nodeId)"
								class="w-full text-left"
								@click="handleMoveClick"
							/>

						</td>

					</tr>

					<!-- Variation row(s) (18 columns) -->

					<VariationRow
						v-else-if="row.type === 'variation' && showVariations"
						:moves="row.moves"
						:is-current-move="isCurrentMove"
						:max-depth="12"
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

interface Props {
	tableRows: TableRow[];
	showVariations: boolean;
	isCurrentMove: (nodeId: NodeId) => boolean;
}

defineProps<Props>();

const emit = defineEmits<{
	"move-select": [moveId: number | undefined];
}>();

const handleMoveClick = (moveId: number | undefined) => {
	emit("move-select", moveId);
};
</script>

