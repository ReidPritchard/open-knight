<template>

	<div class="p-4 bg-base-100 rounded-lg flex flex-col min-h-0">

		<!-- View mode toggle and options -->

		<div class="flex justify-center mb-2 gap-4">

			<div class="join">

				<button
					v-for="mode in ['compact', 'tabular'] as const"
					:key="mode"
					@click="viewMode = mode"
					class="join-item btn btn-xs btn-ghost btn-secondary"
					:class="{ 'btn-active': viewMode === mode }"
				>
					 {{ mode }}
				</button>

			</div>

			<!-- Show variations toggle -->

			<div class="form-control">

				<label class="label cursor-pointer gap-2">

					<span class="label-text text-xs">Show variations</span>

					<input
						type="checkbox"
						v-model="showVariations"
						class="checkbox checkbox-xs"
					/>

				</label>

			</div>

		</div>

		<!-- Current position info -->

		<MoveInfo
			v-if="currentNode"
			:current-node="currentNode"
		/>

		<!-- Move display area -->

		<div
			class="bg-base-200 rounded-lg p-4 flex-1 min-h-0"
			ref="moveContainer"
		>

			<div v-if="rootNode">

				<!-- Compact view -->

				<CompactView
					v-if="viewMode === 'compact'"
					:move-groups="moveGroups"
					:show-variations="showVariations"
					:is-current-move="isCurrentMove"
					@move-select="handleMoveSelect"
				/>

				<!-- Tabular view -->

				<TabularView
					v-if="viewMode === 'tabular'"
					:table-rows="tableRows"
					:show-variations="showVariations"
					:is-current-move="isCurrentMove"
					@move-select="handleMoveSelect"
				/>

			</div>

			<div
				v-else
				class="text-center text-base-content/60 py-8"
			>
				 No moves to display
			</div>

		</div>

	</div>

</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import type { ChessMoveTree } from "../../shared/bindings";
import type { ViewMode } from "../../shared/types";
import { useMoveData } from "../../composables/useMoveData";

import MoveInfo from "./MoveInfo/MoveInfo.vue";
import CompactView from "./CompactView/CompactView.vue";
import TabularView from "./TabularView/TabularView.vue";

interface Props {
	moveTree: ChessMoveTree;
}

const props = defineProps<Props>();
const emit = defineEmits<{
	"select-move": [move_id: number];
	"navigate-start": [];
	"navigate-end": [];
	"navigate-previous": [];
	"navigate-next": [variation_idx: number];
}>();

// State
const viewMode = ref<ViewMode>("tabular");
const showVariations = ref(true);
const moveContainer = ref<HTMLElement | null>(null);

// Use move data composable
const moveTreeComputed = computed(() => props.moveTree);
const {
	rootNode,
	currentNode,
	currentNodeId,
	moveGroups,
	tableRows,
	isCurrentMove,
} = useMoveData(moveTreeComputed);

// Methods
const handleMoveSelect = (moveId: number | undefined): void => {
	if (!moveId) {
		console.warn("Move has no id");
		// FIXME: This can happen when the move hasn't been saved to the database yet
		// we should handle this case better
		return;
	}
	emit("select-move", moveId);
};

// Auto-scroll to current move
watch(currentNodeId, async () => {
	await nextTick();
	if (moveContainer.value) {
		const currentButton = moveContainer.value.querySelector(
			".bg-primary, .bg-secondary",
		);
		if (currentButton) {
			currentButton.scrollIntoView({ behavior: "smooth", block: "center" });
		}
	}
});
</script>

