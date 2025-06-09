<template>

	<div
		v-if="currentNode"
		class="my-4 space-y-2"
	>

		<!-- Current move -->

		<div
			class="flex justify-between gap-2 text-md w-full border-b border-base-300 pb-2"
		>

			<PhCaretRight :size="16" />

			<span class="font-mono cursor-pointer">

				<MoveButton
					v-if="currentMoveData"
					:move-data="currentMoveData"
					:is-current="true"
					:variant="currentMoveData?.isMainLine ? 'main' : 'variation'"
				/>

			</span>

			<PhCaretLeft :size="16" />

		</div>

		<!-- Evaluation display -->

		<div
			v-if="currentEvaluation"
			class="flex items-center gap-2 text-sm"
		>

			<PhChartLine :size="16" />

			<span class="font-mono">{{ formatEvaluation(currentEvaluation) }}</span>

			<span class="text-base-content/60">
				 depth {{ currentEvaluation.depth }}
			</span>

		</div>

		<div
			v-else
			class="flex items-center gap-2 text-sm"
		>

			<span class="text-base-content/60">No evaluation</span>

		</div>

		<!-- Time info -->

		<div
			v-if="currentMove?.time_info"
			class="flex items-center gap-2 text-sm"
		>

			<PhClock :size="16" />

			<span>{{ formatTime(currentMove.time_info.time_spent_ms) }} spent</span>

			<span
				v-if="currentMove.time_info.time_left_ms"
				class="text-base-content/60"
			>
				 ({{ formatTime(currentMove.time_info.time_left_ms) }} left)
			</span>

		</div>

		<div
			v-else
			class="flex items-center gap-2 text-sm"
		>

			<span class="text-base-content/60">No time info</span>

		</div>

		<!-- Annotations -->

		<div
			v-if="currentAnnotations.length > 0"
			class="space-y-2"
		>

			<div
				v-for="annotation in currentAnnotations"
				:key="annotation.id"
				class="bg-base-300 rounded p-2 text-sm flex flex-col gap-2"
			>

				<span class="font-mono flex items-center gap-2">

					<PhChatText :size="14" />

					<textarea
						v-if="annotation.comment"
						class="textarea textarea-ghost w-full resize-none"
						:value="annotation.comment"
						@input="handleCommentInput"
					/>

				</span>

				<span
					v-if="annotation.arrows"
					class="font-mono flex items-center gap-2"
				>

					<PhArrowsOut :size="14" />
					 {{ annotation.arrows }}
				</span>

				<span
					v-if="annotation.highlights"
					class="font-mono flex items-center gap-2"
				>

					<PhArrowsOut :size="14" />
					 {{ annotation.highlights }}
				</span>

			</div>

		</div>

		<div
			v-else
			class="flex items-center gap-2 text-sm"
		>

			<span class="text-base-content/60">No annotations</span>

		</div>

	</div>

</template>

<script setup lang="ts">
import { computed } from "vue";
import {
	PhCaretLeft,
	PhCaretRight,
	PhChartLine,
	PhChatText,
	PhClock,
	PhArrowsOut,
} from "@phosphor-icons/vue";
import type {
	ChessAnnotation,
	ChessEvaluation,
	ChessMove,
	ChessTreeNode,
} from "../../../shared/bindings";
import { useFormatting } from "../../../composables/useFormatting";
import MoveButton from "../MoveButton/MoveButton.vue";
import { MoveData } from "../../../shared/types";

interface Props {
	currentNode: ChessTreeNode | null;
}

const props = defineProps<Props>();

const { formatEvaluation, formatTime } = useFormatting();

const currentMove = computed(
	(): ChessMove | null | undefined => props.currentNode?.game_move,
);

const currentMoveData = computed((): MoveData | null => {
	if (!currentMove.value) return null;
	// Create a partial MoveData object from the Chess Move
	const moveData: MoveData = {
		nodeId: {
			idx: props.currentNode?.parent_id?.idx || 0,
			version: props.currentNode?.parent_id?.version || 0,
		},
		node: props.currentNode as ChessTreeNode,
		move: currentMove.value,
		san: currentMove.value.san,
		plyNumber: currentMove.value.ply_number,
		moveNumber: Math.ceil(currentMove.value.ply_number / 2),
		showNumber: true,
		isWhite: currentMove.value.ply_number % 2 === 1,
		isMainLine: true,
		isVariation: false,
		depth: 0,
		parentMoveNumber: props.currentNode?.parent_id?.idx || null,
	};

	// Check the current node for annotations, particularly comments
	const currentNodeAnnotations = props.currentNode?.game_move?.annotations;

	if (currentNodeAnnotations) {
		// Find any NAGs (Numeric Annotation Glyphs) in comments ($1, $2, etc.)
		// TODO: Add support for more NAGs. For now, we only support $1-$7
		const nagRegex = /\$(\d)/g;
		const nagMatches = currentNodeAnnotations.reduce(
			(acc: string[], annotation) => {
				const nagMatch = annotation.comment?.match(nagRegex);
				if (nagMatch) {
					acc.push(nagMatch[1]);
				}
				return acc;
			},
			[],
		);
		// Add mapped NAGs to move's san string?
		const symbols = ["!", "?", "!!", "??", "!?", "?!", "□"];
		const nagSymbols = nagMatches.map((nag) => symbols[Number(nag) - 1]);
		console.log(nagSymbols);
		moveData.san = `${moveData.san} ${nagSymbols.join(" ")}`;

		// Find any highlights in comments (e.g. [a1-c3])
		const highlightRegex = /\[([a-h][1-8]-[a-h][1-8])\]/g;
		const highlightMatches = currentNodeAnnotations.map((annotation) =>
			annotation.comment?.match(highlightRegex),
		);
		console.log(highlightMatches);
	}

	return moveData;
});

const currentPosition = computed(() => props.currentNode?.position);

const currentEvaluation = computed((): ChessEvaluation | null => {
	if (!currentPosition.value?.evaluations?.length) return null;
	return currentPosition.value.evaluations.reduce(
		(
			best: ChessEvaluation | null,
			evaluation: ChessEvaluation,
		): ChessEvaluation => {
			if (!best || (evaluation.depth && evaluation.depth > (best.depth || 0)))
				return evaluation;
			return best;
		},
		null,
	);
});

const currentAnnotations = computed((): ChessAnnotation[] => {
	if (!currentMove.value?.annotations) return [];
	return currentMove.value.annotations;
});

const handleCommentInput = (event: Event) => {
	const textarea = event.target as HTMLTextAreaElement;
	textarea.value = textarea.value.replace(/[^a-zA-Z0-9\s]/g, "");
};
</script>

