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
					 {{ currentMove?.san ?? "Initial position" }}
				</span>

				<PhCaretLeft :size="16" />

			</div>

			<!-- Evaluation display -->

			<div
				v-if="currentEvaluation"
				class="flex items-center gap-2 text-sm"
			>

				<PhChartLine :size="16" />

				<span class="font-mono">
					 {{ formatEvaluation(currentEvaluation) }}
				</span>

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

				<span>
					 {{ formatTime(currentMove.time_info.time_spent_ms) }} spent
				</span>

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
					class="bg-base-300 rounded p-2 text-sm"
				>

					<PhChatText
						:size="14"
						class="inline mr-1"
					/>
					 {{ annotation.comment }}
				</div>

			</div>

			<div
				v-else
				class="flex items-center gap-2 text-sm"
			>

				<span class="text-base-content/60">No annotations</span>

			</div>

		</div>

		<!-- Move display area -->

		<div
			class="bg-base-200 rounded-lg p-4 flex-1 min-h-0"
			ref="moveContainer"
		>

			<div v-if="rootNode">

				<!-- Compact view (optimized for long games) -->

				<div
					v-if="viewMode === 'compact'"
					class="space-y-2"
				>

					<div
						v-for="(group, index) in moveGroups"
						:key="index"
						class="space-y-1"
					>

						<!-- Main line moves -->

						<div class="flex flex-wrap gap-1">

							<button
								v-for="moveData in group.mainMoves"
								:key="moveData.nodeId.idx"
								@click="handleMoveSelect(moveData.move?.id)"
								class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer"
								:class="{
									'bg-primary text-primary-content': isCurrentMove(
										moveData.nodeId,
									),
									'hover:bg-base-300': !isCurrentMove(moveData.nodeId),
								}"
							>

								<span
									v-if="moveData.showNumber"
									class="font-bold"
								>
									 {{ moveData.moveNumber }}.
								</span>
								 {{ moveData.san }}
							</button>

						</div>

						<!-- Variations -->

						<div
							v-if="showVariations && group.variations.length > 0"
							class="ml-4 space-y-1"
						>

							<div
								v-for="(variation, varIndex) in group.variations"
								:key="varIndex"
								class="flex flex-wrap gap-1 items-center"
							>

								<PhGitBranch
									size="12"
									class="text-base-content/60"
								/>

								<span class="text-xs text-base-content/60">(</span>

								<button
									v-for="moveData in variation"
									:key="moveData.nodeId.idx"
									@click="handleMoveSelect(moveData.move?.id)"
									class="px-1.5 py-0.5 rounded text-xs font-mono transition-colors cursor-pointer border border-base-300"
									:class="{
										'bg-secondary text-secondary-content border-secondary':
											isCurrentMove(moveData.nodeId),
										'hover:bg-base-300': !isCurrentMove(moveData.nodeId),
										'bg-base-100': !isCurrentMove(moveData.nodeId),
									}"
								>

									<span
										v-if="moveData.showNumber"
										class="font-bold"
									>
										 {{ moveData.moveNumber }}.
									</span>
									 {{ moveData.san }}
								</button>

								<span class="text-xs text-base-content/60">)</span>

							</div>

						</div>

					</div>

				</div>

				<!-- Tabular view -->

				<div
					v-if="viewMode === 'tabular'"
					class="overflow-x-auto"
				>

					<table class="table table-xs">

						<tbody>

							<template
								v-for="(row, index) in tableRows"
								:key="index"
							>

								<!-- Main move row -->

								<tr v-if="row.type === 'move'">

									<td class="font-bold text-right">{{ row.number }}.</td>

									<td>

										<button
											v-if="row.white"
											@click="handleMoveSelect(row.white.move?.id)"
											class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer w-full text-left"
											:class="{
												'bg-primary text-primary-content': isCurrentMove(
													row.white.nodeId,
												),
												'hover:bg-base-300': !isCurrentMove(row.white.nodeId),
											}"
										>
											 {{ row.white.san }}
										</button>

									</td>

									<td>

										<button
											v-if="row.black"
											@click="handleMoveSelect(row.black.move?.id)"
											class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer w-full text-left"
											:class="{
												'bg-primary text-primary-content': isCurrentMove(
													row.black.nodeId,
												),
												'hover:bg-base-300': !isCurrentMove(row.black.nodeId),
											}"
										>
											 {{ row.black.san }}
										</button>

									</td>

								</tr>

								<!-- Variation row -->

								<tr
									v-else-if="row.type === 'variation' && showVariations"
									class=""
								>

									<td></td>

									<td
										colspan="2"
										class="pl-2"
									>

										<div class="flex items-center gap-1 flex-wrap">

											<PhGitBranch
												:size="12"
												class="text-base-content/60"
											/>

											<span class="text-xs text-base-content/60">(</span>

											<button
												v-for="moveData in row.moves"
												:key="moveData.nodeId.idx"
												@click="handleMoveSelect(moveData.move?.id)"
												class="px-1.5 py-0.5 rounded text-xs font-mono transition-colors cursor-pointer border border-base-300"
												:class="{
													'bg-secondary text-secondary-content border-secondary':
														isCurrentMove(moveData.nodeId),
													'hover:bg-base-300': !isCurrentMove(moveData.nodeId),
													'bg-base-100': !isCurrentMove(moveData.nodeId),
												}"
											>

												<span
													v-if="moveData.showNumber"
													class="font-bold"
												>
													 {{ moveData.moveNumber }}.
												</span>
												 {{ moveData.san }}
											</button>

											<span class="text-xs text-base-content/60">)</span>

										</div>

									</td>

								</tr>

							</template>

						</tbody>

					</table>

				</div>

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
import {
	PhCaretLeft,
	PhCaretRight,
	PhChartLine,
	PhChatText,
	PhClock,
	PhGitBranch,
} from "@phosphor-icons/vue";
import { computed, nextTick, ref, watch } from "vue";
import type {
	ChessAnnotation,
	ChessEvaluation,
	ChessMove,
	ChessMoveTree,
	ChessPosition,
	ChessTreeNode,
} from "../../shared/bindings";

// Types
interface MoveData {
	nodeId: { idx: number; version: number };
	node: ChessTreeNode;
	move?: ChessMove;
	san: string;
	plyNumber: number;
	moveNumber: number;
	showNumber: boolean;
	isWhite: boolean;
	isMainLine: boolean;
	isVariation: boolean;
	depth: number;
	parentMoveNumber: number | null;
}

interface MoveGroup {
	mainMoves: MoveData[];
	variations: MoveData[][];
}

interface TableMoveRow {
	type: "move";
	number: number;
	white?: MoveData;
	black?: MoveData;
}

interface TableVariationRow {
	type: "variation";
	moves: MoveData[];
}

type TableRow = TableMoveRow | TableVariationRow;

const props = defineProps<{
	moveTree: ChessMoveTree;
}>();

const emit = defineEmits<{
	"select-move": [move_id: number];
	"navigate-start": [];
	"navigate-end": [];
	"navigate-previous": [];
	"navigate-next": [variation_idx: number];
}>();

// State
// TODO: Track this in the UI store so it persists when the component is destroyed
const viewMode = ref<"compact" | "tabular">("tabular");
const showVariations = ref(true);
const moveContainer = ref<HTMLElement | null>(null);

// Computed properties
const rootNode = computed((): ChessTreeNode | null => {
	if (!props.moveTree.root_id) return null;
	return props.moveTree.nodes[props.moveTree.root_id.idx]?.value || null;
});

const currentNodeId = computed(() => props.moveTree.current_node_id);

const currentNode = computed((): ChessTreeNode | null => {
	if (!currentNodeId.value) return null;
	return props.moveTree.nodes[currentNodeId.value.idx]?.value || null;
});

const currentMove = computed(
	(): ChessMove | null | undefined => currentNode.value?.game_move,
);

const currentPosition = computed(
	(): ChessPosition | undefined => currentNode.value?.position,
);

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
	return currentMove.value.annotations.filter(
		(a: ChessAnnotation) => a.comment,
	);
});

// Enhanced move tree flattening that preserves variation structure
const structuredMoves = computed((): MoveData[] => {
	if (!rootNode.value) return [];

	const visited = new Set<string>();

	function traverse(
		node: ChessTreeNode,
		isMainLine = true,
		depth = 0,
		parentMoveNumber: number | null = null,
	): MoveData[] {
		const nodeWrapper = props.moveTree.nodes.find((n) => n.value === node);
		if (!nodeWrapper) return [];

		const nodeId = {
			idx: props.moveTree.nodes.indexOf(nodeWrapper),
			version: nodeWrapper.version,
		};

		// Avoid cycles
		const nodeKey = `${nodeId.idx}-${nodeId.version}`;
		if (visited.has(nodeKey)) return [];
		visited.add(nodeKey);

		const moves: MoveData[] = [];

		if (node.game_move) {
			const moveData: MoveData = {
				nodeId,
				node,
				move: node.game_move,
				san: node.game_move.san,
				plyNumber: node.game_move.ply_number,
				moveNumber: Math.ceil(node.game_move.ply_number / 2),
				showNumber: node.game_move.ply_number % 2 === 1,
				isWhite: node.game_move.ply_number % 2 === 1,
				isMainLine,
				isVariation: !isMainLine,
				depth,
				parentMoveNumber,
			};
			moves.push(moveData);
		}

		// Process children
		const children = (node.children_ids || [])
			.map((childId) => props.moveTree.nodes[childId.idx]?.value)
			.filter(Boolean);

		if (children.length > 0) {
			// First child continues main line or current variation
			const firstChild = children[0];
			if (firstChild) {
				const mainChild = traverse(
					firstChild,
					isMainLine,
					depth,
					node.game_move
						? Math.ceil(node.game_move.ply_number / 2)
						: parentMoveNumber,
				);
				moves.push(...mainChild);
			}

			// Other children are variations
			for (let i = 1; i < children.length; i++) {
				const child = children[i];
				if (child) {
					const variation = traverse(
						child,
						false,
						depth + 1,
						node.game_move
							? Math.ceil(node.game_move.ply_number / 2)
							: parentMoveNumber,
					);
					moves.push(...variation);
				}
			}
		}

		return moves;
	}

	return traverse(rootNode.value);
});

// Group moves for compact view with variations
const moveGroups = computed((): MoveGroup[] => {
	const mainLineMoves = structuredMoves.value.filter((m) => m.isMainLine);
	const variations = structuredMoves.value.filter((m) => m.isVariation);

	const groups: MoveGroup[] = [];
	const movesPerGroup = 6; // Reduced to leave space for variations

	for (let i = 0; i < mainLineMoves.length; i += movesPerGroup) {
		const mainMoves = mainLineMoves.slice(i, i + movesPerGroup);
		const lastMoveNumber = mainMoves[mainMoves.length - 1]?.moveNumber;

		// Find variations that branch from moves in this group
		const groupVariations: MoveData[][] = [];
		if (lastMoveNumber) {
			const variationsByParent: Record<number, MoveData[]> = {};
			for (const v of variations) {
				if (
					v.parentMoveNumber &&
					v.parentMoveNumber <= lastMoveNumber &&
					v.parentMoveNumber >= (mainMoves[0]?.moveNumber || 1)
				) {
					if (!variationsByParent[v.parentMoveNumber]) {
						variationsByParent[v.parentMoveNumber] = [];
					}
					variationsByParent[v.parentMoveNumber].push(v);
				}
			}

			// Group consecutive variation moves
			for (const parentVars of Object.values(variationsByParent)) {
				const variationChains: MoveData[][] = [];
				let currentChain: MoveData[] = [];

				for (const v of parentVars) {
					if (
						currentChain.length === 0 ||
						v.plyNumber === currentChain[currentChain.length - 1].plyNumber + 1
					) {
						currentChain.push(v);
					} else {
						if (currentChain.length > 0)
							variationChains.push([...currentChain]);
						currentChain = [v];
					}
				}

				if (currentChain.length > 0) variationChains.push(currentChain);
				groupVariations.push(...variationChains);
			}
		}

		groups.push({
			mainMoves,
			variations: groupVariations,
		});
	}

	return groups;
});

// Enhanced table rows that include variations
const tableRows = computed((): TableRow[] => {
	const mainLineMoves = structuredMoves.value.filter((m) => m.isMainLine);
	const variations = structuredMoves.value.filter((m) => m.isVariation);
	const rows: TableRow[] = [];

	// Group main moves by move number
	const movesByNumber: Record<
		number,
		{ white: MoveData | null; black: MoveData | null }
	> = {};
	for (const move of mainLineMoves) {
		if (!movesByNumber[move.moveNumber]) {
			movesByNumber[move.moveNumber] = { white: null, black: null };
		}
		if (move.isWhite) {
			movesByNumber[move.moveNumber].white = move;
		} else {
			movesByNumber[move.moveNumber].black = move;
		}
	}

	// Create table rows
	for (const [number, moves] of Object.entries(movesByNumber)) {
		rows.push({
			type: "move",
			number: Number.parseInt(number),
			white: moves.white || undefined,
			black: moves.black || undefined,
		});

		// Add variations that branch from this move
		const moveVariations = variations.filter(
			(v) => v.parentMoveNumber === Number.parseInt(number),
		);

		if (moveVariations.length > 0) {
			// Group variations into chains
			const variationChains: MoveData[][] = [];
			let currentChain: MoveData[] = [];

			for (const v of moveVariations) {
				if (
					currentChain.length === 0 ||
					v.plyNumber === currentChain[currentChain.length - 1].plyNumber + 1
				) {
					currentChain.push(v);
				} else {
					if (currentChain.length > 0) variationChains.push([...currentChain]);
					currentChain = [v];
				}
			}

			if (currentChain.length > 0) variationChains.push(currentChain);

			for (const chain of variationChains) {
				rows.push({
					type: "variation",
					moves: chain,
				});
			}
		}
	}

	return rows;
});

// Methods
const handleMoveSelect = (moveId: number | undefined): void => {
	if (!moveId) {
		console.warn("Move has no id");
		// FIXME: I think this can happen when the move hasn't been saved to the database yet
		// we should handle this case better
		return;
	}
	emit("select-move", moveId);
};

const isCurrentMove = (nodeId: { idx: number; version: number }): boolean => {
	return (
		currentNodeId.value !== undefined &&
		currentNodeId.value.idx === nodeId.idx &&
		currentNodeId.value.version === nodeId.version
	);
};

const formatEvaluation = (evaluation: ChessEvaluation): string => {
	if (!evaluation || evaluation.score == null) return "?";
	const score = evaluation.score / 100;
	if (evaluation.eval_type === "mate") {
		return `#${evaluation.score > 0 ? "+" : ""}${evaluation.score}`;
	}
	return `${score > 0 ? "+" : ""}${score.toFixed(2)}`;
};

const formatTime = (ms: number | null | undefined): string => {
	if (!ms) return "0:00";
	const seconds = Math.floor(ms / 1000);
	const minutes = Math.floor(seconds / 60);
	const remainingSeconds = seconds % 60;
	return `${minutes}:${remainingSeconds.toString().padStart(2, "0")}`;
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

