<template>

	<Story title="Move Display/Move Info">

		<template #controls>

			<div class="flex flex-row gap-2">

				<HstButton
					color="primary"
					class="htw-p-2"
					@click="addAnnotation"
				>
					 Add Annotation
				</HstButton>

				<HstButton
					class="htw-p-2"
					@click="removeAnnotation"
				>
					 Remove Annotation
				</HstButton>

			</div>

			<HstTextarea
				v-model="currentNode.game_move.annotations[0].comment"
				title="Comment"
			/>

		</template>

		<Variant title="Default">

			<MoveInfo
				:current-node="currentNode"
				@update-move-comment="
					logEvent('update-move-comment', {
						comment: $event,
					})
				"
			/>

		</Variant>

	</Story>

</template>

<script setup lang="ts">
import { reactive } from "vue";
import MoveInfo from "./MoveInfo.vue";
import { logEvent } from "histoire/client";

const currentNode = reactive({
	position: {
		id: 0,
		fen: "",
		evaluations: [
			{
				score: 123,
				eval_type: "cp",
				is_mate: false,
				depth: 99,
				engine: "Stockfish",
				principal_variation: ["e2e4", "e7e5"],
			},
		],
		variant: null,
	},
	game_move: {
		id: 0,
		game_id: 0,
		ply_number: 1,
		san: "e4",
		uci: "e2e4",
		position: null,
		annotations: [
			{
				id: 0,
				comment: "A text annotation for the move",
				arrows: null,
				highlights: null,
			},
		],
		time_info: {
			time_spent_ms: 123456,
			time_left_ms: 789012,
		},
		parent_move_id: null,
	},
	parent_id: null,
	children_ids: [],
});

const addAnnotation = () => {
	currentNode.game_move.annotations.push({
		id: currentNode.game_move.annotations.length + 1,
		comment: "A new annotation",
		arrows: null,
		highlights: null,
	});
};

const removeAnnotation = () => {
	if (currentNode.game_move.annotations.length > 0) {
		currentNode.game_move.annotations.pop();
	}
};
</script>

