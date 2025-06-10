<template>

	<!-- TODO: Only create a collapse if there are nested variations or >5 moves -->

	<details
		class="collapse w-full"
		:open="detailsOpen"
		@toggle="handleToggle"
	>

		<summary class="collapse-title font-semibold w-full after:hidden">

			<!-- TODO: add variation preview here -->

			<div
				class="flex flex-row flex-nowrap justify-center items-center w-full"
				:class="{
					'bg-base-100': !detailsOpen,
					'bg-success': detailsOpen,
				}"
			>

				<div
					class="text-sm"
					:class="{
						'text-base-content/60': !detailsOpen,
						'text-success-content': detailsOpen,
					}"
				>

					<PhGitBranch :size="12" />

				</div>

				<!-- First move of variation -->

				<div
					class="w-1/4"
					:class="{
						'hidden': !detailsOpen,
					}"
				>

					<MoveButton
						:move-data="nonNestedMoves[0]"
						:is-current="isCurrentMove(nonNestedMoves[0].nodeId)"
						variant="variation"
						:size="size"
						:disabled="true"
					/>

				</div>

			</div>

		</summary>

		<div class="collapse-content">

			<ul class="w-full">

				<li>

					<div class="flex flex-wrap gap-1 items-center">

						<span
							v-if="showOpenParen"
							class="text-xs text-base-content/60"
						>
							 (
						</span>

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

						<span
							v-if="showCloseParen"
							class="text-xs text-base-content/60"
						>
							 )
						</span>

					</div>

				</li>

				<li v-if="nestedVariations.length > 0">

					<template
						v-for="nestedVariation in nestedVariations"
						:key="`nested-${nestedVariation.depth}`"
					>

						<VariationRow
							:moves="nestedVariation.moves"
							:is-current-move="isCurrentMove"
							:size="size"
							:depth="nestedVariation.depth + 1"
							@move-click="handleMoveClick"
						/>

					</template>

				</li>

				<li v-if="afterMoves.length > 0">

					<div class="flex flex-wrap gap-1 items-center">

						<template
							v-for="move in afterMoves"
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

				</li>

			</ul>

		</div>

	</details>

</template>

<script setup lang="ts">
import { computed, ref } from "vue";
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

// TODO: Add support for collapsing variations that are multi-line
// when the variation is collapsed, the variation should be rendered as a single row (first n moves)
// and any variations within the collapsed variation should not be rendered

// Handle nested variations
const isNestedVariation = (move: VariationMove): move is TableVariationRow => {
	return "type" in move && move.type === "variation";
};

// TODO: support multiple nested variations in a single line/variation/set of moves
// This creates a more sophisticated structure to show where variations start and end
// by splitting regular moves around nested variation points
//
// Split regular moves by nested variation location
// ex. M1 -> M2 -> M3 -> M4
//           M2 -> VM3
// renders as:
// Row 1: (M1 -> M2 -> M3
// Row 2 (indented): (M2 -> VM3)
// Row 3: M3 -> M4)
// Note: row 1 opens with a parenthesis which is closed by row 3 as together they form
// the main variation line. Row 2 is indented and has surrounding parentheses
// to show that it is nested within row 1/3
const moveSegments = computed(() => {
	const segments: {
		beforeMoves: MoveData[];
		nestedVariations: (TableVariationRow & { depth: number })[];
		afterMoves: MoveData[];
	}[] = [];

	let currentSegment = {
		beforeMoves: [] as MoveData[],
		nestedVariations: [] as (TableVariationRow & { depth: number })[],
		afterMoves: [] as MoveData[],
	};

	let inAfterSection = false;

	for (const move of props.moves) {
		if (isNestedVariation(move)) {
			// Found a nested variation - this splits the current segment
			currentSegment.nestedVariations.push({
				...move,
				depth: props.depth + 1,
			});
			inAfterSection = true;
		} else {
			// Regular move
			if (inAfterSection) {
				currentSegment.afterMoves.push(move);
			} else {
				currentSegment.beforeMoves.push(move);
			}
		}
	}

	// Only add the segment if it has content
	if (
		currentSegment.beforeMoves.length > 0 ||
		currentSegment.nestedVariations.length > 0 ||
		currentSegment.afterMoves.length > 0
	) {
		segments.push(currentSegment);
	}

	return segments;
});

// Extract moves that appear before any nested variations for the main row
const nonNestedMoves = computed(() => {
	const segment = moveSegments.value[0];
	if (!segment) return [];

	// For the main row, we show moves before nested variations
	// If there are nested variations, we don't close the parenthesis (it continues in afterMoves)
	// If there are no nested variations, we show all moves and close the parenthesis
	if (segment.nestedVariations.length > 0) {
		return segment.beforeMoves;
	} else {
		return [...segment.beforeMoves, ...segment.afterMoves];
	}
});

// Extract nested variations to render as separate rows
const nestedVariations = computed(() => {
	const segment = moveSegments.value[0];
	return segment ? segment.nestedVariations : [];
});

// Extract moves that come after nested variations for continuation rows
const afterMoves = computed(() => {
	const segment = moveSegments.value[0];
	return segment && segment.nestedVariations.length > 0
		? segment.afterMoves
		: [];
});

// Determine if we should show opening parenthesis (always true for variation rows)
const showOpenParen = computed(() => true);

// Determine if we should show closing parenthesis
// (only if there are no nested variations or this is a continuation row)
const showCloseParen = computed(() => {
	const segment = moveSegments.value[0];
	return !segment || segment.nestedVariations.length === 0;
});

// State tracking if the variation row is open
const detailsOpen = ref(false);

const handleToggle = () => {
	detailsOpen.value = !detailsOpen.value;
};
</script>

