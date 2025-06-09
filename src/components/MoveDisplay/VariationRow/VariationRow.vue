<template>

	<div class="flex flex-wrap gap-1 items-center">

		<PhGitBranch
			:size="12"
			class="text-base-content/60"
		/>

		<span class="text-xs text-base-content/60">(</span>

		<MoveButton
			v-for="moveData in moves"
			:key="moveData.nodeId.idx"
			:move-data="moveData"
			:is-current="isCurrentMove(moveData.nodeId)"
			variant="variation"
			:size="size"
			@click="handleMoveClick"
		/>

		<span class="text-xs text-base-content/60">)</span>

	</div>

</template>

<script setup lang="ts">
import { PhGitBranch } from "@phosphor-icons/vue";
import MoveButton from "../MoveButton/MoveButton.vue";
import type { MoveData, NodeId } from "../../../shared/types";

interface Props {
	moves: MoveData[];
	isCurrentMove: (nodeId: NodeId) => boolean;
	size?: "sm" | "xs";
}

withDefaults(defineProps<Props>(), {
	size: "xs",
});

const emit = defineEmits<{
	"move-click": [moveId: number | undefined];
}>();

const handleMoveClick = (moveId: number | undefined) => {
	emit("move-click", moveId);
};
</script>

