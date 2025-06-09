<template>

	<!-- FIXME: Create a new row for each nested variation (one that is not within this row) -->

	<tr>

		<td
			colspan="2"
			:class="`pl-${depth * 4}`"
		>

			<div class="flex flex-wrap gap-1 items-center">

				<PhGitBranch
					:size="12"
					class="text-base-content/60"
				/>

				<span class="text-xs text-base-content/60">(</span>

				<!-- Only display moves that are not nested variations -->

				<template
					v-for="move in nonNestedMoves"
					:key="move.nodeId.idx"
				>

					<MoveButton
						:move-data="move"
						:is-current="isCurrentMove(move.nodeId)"
						variant="variation"
						:size="size"
						@click="handleMoveClick"
					/>

				</template>

				<span class="text-xs text-base-content/60">)</span>

			</div>

		</td>

	</tr>

	<!-- Render nested variations as separate rows -->

	<template
		v-for="nestedVariation in nestedVariations"
		:key="`nested-${nestedVariation.depth}`"
	>

		<VariationRow
			:moves="nestedVariation.moves"
			:is-current-move="isCurrentMove"
			:size="size"
			:depth="nestedVariation.depth"
			@move-click="handleMoveClick"
		/>

	</template>

</template>

<script setup lang="ts">
import { computed } from "vue";
import { PhGitBranch } from "@phosphor-icons/vue";
import MoveButton from "../MoveButton/MoveButton.vue";
import type {
	VariationMove,
	NodeId,
	TableVariationRow,
	MoveData,
} from "../../../shared/types";

interface Props {
	moves: VariationMove[];
	isCurrentMove: (nodeId: NodeId) => boolean;
	size?: "sm" | "xs";
	depth?: number;
}

const props = withDefaults(defineProps<Props>(), {
	size: "xs",
	depth: 0,
});

const emit = defineEmits<{
	"move-click": [moveId: number | undefined];
}>();

const handleMoveClick = (moveId: number | undefined) => {
	emit("move-click", moveId);
};

// handle nested variations
const isNestedVariation = (move: VariationMove): move is TableVariationRow => {
	return "type" in move && move.type === "variation";
};

// Separate moves into nested variations and regular moves
const nonNestedMoves = computed((): MoveData[] => {
	return props.moves.filter(
		(move): move is MoveData => !isNestedVariation(move),
	);
});

const nestedVariations = computed((): TableVariationRow[] => {
	return props.moves.filter(isNestedVariation);
});
</script>

