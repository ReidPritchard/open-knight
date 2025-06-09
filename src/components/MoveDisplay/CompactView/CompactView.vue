<template>

	<div class="space-y-2">

		<div
			v-for="(group, index) in moveGroups"
			:key="index"
			class="space-y-1"
		>

			<!-- Main line moves -->

			<div class="flex flex-wrap gap-1">

				<MoveButton
					v-for="moveData in group.mainMoves"
					:key="moveData.nodeId.idx"
					:move-data="moveData"
					:is-current="isCurrentMove(moveData.nodeId)"
					@click="handleMoveClick"
				/>

			</div>

			<!-- Variations -->

			<div
				v-if="showVariations && group.variations.length > 0"
				class="ml-4 space-y-1"
			>

				<VariationRow
					v-for="(variation, varIndex) in group.variations"
					:key="varIndex"
					:moves="variation"
					:is-current-move="isCurrentMove"
					@move-click="handleMoveClick"
				/>

			</div>

		</div>

	</div>

</template>

<script setup lang="ts">
import MoveButton from "../MoveButton/MoveButton.vue";
import VariationRow from "../VariationRow/VariationRow.vue";
import type { MoveGroup, NodeId } from "../../../shared/types";

interface Props {
	moveGroups: MoveGroup[];
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

