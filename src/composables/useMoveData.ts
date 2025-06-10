import { computed, type ComputedRef } from "vue";
import type { ChessMoveTree, ChessTreeNode } from "../shared/bindings";
import type {
	MoveData,
	MoveGroup,
	TableRow,
	VariationMove,
	NodeId,
} from "../shared/types";

export function useMoveData(moveTree: ComputedRef<ChessMoveTree>) {
	// Helper to get node by ID
	const getNode = (nodeId: NodeId | undefined): ChessTreeNode | null => {
		if (!nodeId) return null;
		return moveTree.value.nodes[nodeId.idx]?.value || null;
	};

	// Helper to find node ID from node
	const findNodeId = (node: ChessTreeNode): NodeId | null => {
		const index = moveTree.value.nodes.findIndex((n) => n.value === node);
		if (index === -1) return null;
		const wrapper = moveTree.value.nodes[index];
		return { idx: index, version: wrapper.version };
	};

	// Core computed properties
	const rootNode = computed(() => getNode(moveTree.value.root_id));
	const currentNodeId = computed(() => moveTree.value.current_node_id);
	const currentNode = computed(() => getNode(currentNodeId.value));

	// Helper to create MoveData from node
	const createMoveData = (
		node: ChessTreeNode,
		nodeId: NodeId,
		isMainLine: boolean,
		depth: number,
		parentMoveNumber: number | null,
	): MoveData | null => {
		if (!node.game_move) return null;

		const move = node.game_move;
		return {
			nodeId,
			node,
			move,
			san: move.san,
			plyNumber: move.ply_number,
			moveNumber: Math.ceil(move.ply_number / 2),
			showNumber: move.ply_number % 2 === 1,
			isWhite: move.ply_number % 2 === 1,
			isMainLine,
			isVariation: !isMainLine,
			depth,
			parentMoveNumber,
		};
	};

	// Simplified tree traversal
	const structuredMoves = computed((): MoveData[] => {
		if (!rootNode.value) return [];

		const moves: MoveData[] = [];
		const visited = new Set<string>();

		const traverse = (
			node: ChessTreeNode,
			isMainLine = true,
			depth = 0,
			parentMoveNumber: number | null = null,
		): void => {
			const nodeId = findNodeId(node);
			if (!nodeId) return;

			// Cycle detection
			const key = `${nodeId.idx}-${nodeId.version}`;
			if (visited.has(key)) return;
			visited.add(key);

			// Add current move
			const moveData = createMoveData(
				node,
				nodeId,
				isMainLine,
				depth,
				parentMoveNumber,
			);
			if (moveData) {
				moves.push(moveData);
				parentMoveNumber = moveData.moveNumber;
			}

			// Process children
			const children = (node.children_ids || [])
				.map((id) => getNode(id))
				.filter((n): n is ChessTreeNode => n !== null);

			// First child continues the current line
			if (children[0]) {
				traverse(children[0], isMainLine, depth, parentMoveNumber);
			}

			// Other children are variations
			for (let i = 1; i < children.length; i++) {
				traverse(children[i], false, depth + 1, parentMoveNumber);
			}
		};

		traverse(rootNode.value);
		return moves;
	});

	// Simplified move grouping
	const moveGroups = computed((): MoveGroup[] => {
		const mainLine = structuredMoves.value.filter((m) => m.isMainLine);
		const variations = structuredMoves.value.filter((m) => m.isVariation);

		const groups: MoveGroup[] = [];
		const movesPerGroup = 6;

		for (let i = 0; i < mainLine.length; i += movesPerGroup) {
			const mainMoves = mainLine.slice(i, i + movesPerGroup);
			const firstMove = mainMoves[0];
			const lastMove = mainMoves[mainMoves.length - 1];

			if (!firstMove || !lastMove) continue;

			// Find variations in this range
			const groupVariations = variations.filter(
				(v) =>
					v.parentMoveNumber !== null &&
					v.parentMoveNumber >= firstMove.moveNumber &&
					v.parentMoveNumber <= lastMove.moveNumber,
			);

			// Group consecutive variations by parent and continuity
			const variationChains: MoveData[][] = [];
			const processed = new Set<string>();

			for (const variation of groupVariations) {
				const key = `${variation.nodeId.idx}-${variation.nodeId.version}`;
				if (processed.has(key)) continue;

				const chain: MoveData[] = [variation];
				processed.add(key);

				// Collect consecutive moves in this variation
				let current = variation;
				while (true) {
					const nextMove = groupVariations.find(
						(v) =>
							v.plyNumber === current.plyNumber + 1 &&
							v.depth === current.depth &&
							!processed.has(`${v.nodeId.idx}-${v.nodeId.version}`),
					);

					if (!nextMove) break;

					chain.push(nextMove);
					processed.add(`${nextMove.nodeId.idx}-${nextMove.nodeId.version}`);
					current = nextMove;
				}

				variationChains.push(chain);
			}

			groups.push({
				mainMoves,
				variations: variationChains,
			});
		}

		return groups;
	});

	// Helper to collect a complete variation line
	const collectVariationLine = (
		startMove: MoveData,
		allVariations: MoveData[],
	): VariationMove[] => {
		const line: VariationMove[] = [startMove];
		const processedIds = new Set<string>([
			`${startMove.nodeId.idx}-${startMove.nodeId.version}`,
		]);

		let currentNode = startMove.node;
		let currentDepth = startMove.depth;

		while (currentNode.children_ids && currentNode.children_ids.length > 0) {
			const children = currentNode.children_ids
				.map((id) => ({ id, node: getNode(id) }))
				.filter(
					(c): c is { id: NodeId; node: ChessTreeNode } => c.node !== null,
				);

			if (children.length === 0) break;

			// First child continues the variation
			const [firstChild, ...otherChildren] = children;

			if (firstChild.node.game_move) {
				const firstChildMove = createMoveData(
					firstChild.node,
					firstChild.id,
					false,
					currentDepth,
					Math.ceil(firstChild.node.game_move.ply_number / 2) - 1,
				);

				if (firstChildMove) {
					line.push(firstChildMove);
					processedIds.add(
						`${firstChildMove.nodeId.idx}-${firstChildMove.nodeId.version}`,
					);
				}
			}

			// Other children are nested variations
			for (const { id, node } of otherChildren) {
				if (!node.game_move) continue;

				const nestedMove = createMoveData(
					node,
					id,
					false,
					currentDepth + 1,
					firstChild.node.game_move
						? Math.ceil(firstChild.node.game_move.ply_number / 2)
						: currentNode.game_move
							? Math.ceil(currentNode.game_move.ply_number / 2)
							: null,
				);

				if (nestedMove) {
					const nestedLine = collectVariationLine(nestedMove, allVariations);
					line.push({
						type: "variation",
						moves: nestedLine,
						depth: currentDepth + 1,
						collapsible: nestedLine.length > 3,
					});
				}
			}

			currentNode = firstChild.node;
		}

		return line;
	};

	// Simplified table rows generation
	const tableRows = computed((): TableRow[] => {
		const mainLine = structuredMoves.value.filter((m) => m.isMainLine);
		const variations = structuredMoves.value.filter((m) => m.isVariation);
		const rows: TableRow[] = [];

		// Group main line by move number
		const movesByNumber = new Map<
			number,
			{ white?: MoveData; black?: MoveData }
		>();

		for (const move of mainLine) {
			const entry = movesByNumber.get(move.moveNumber) || {};
			if (move.isWhite) {
				entry.white = move;
			} else {
				entry.black = move;
			}
			movesByNumber.set(move.moveNumber, entry);
		}

		// Process each move number
		const processedVariations = new Set<string>();

		for (const [moveNumber, moves] of movesByNumber) {
			// Add the main line row
			rows.push({
				type: "move",
				number: moveNumber,
				white: moves.white,
				black: moves.black,
			});

			// Find variations branching from this move
			const branchingVariations = variations.filter(
				(v) =>
					v.parentMoveNumber === moveNumber &&
					v.depth === 1 &&
					!processedVariations.has(`${v.nodeId.idx}-${v.nodeId.version}`),
			);

			// Process each variation line
			for (const variation of branchingVariations) {
				const variationLine = collectVariationLine(variation, variations);

				// Mark all moves in this line as processed
				const markProcessed = (moves: VariationMove[]) => {
					for (const move of moves) {
						if ("nodeId" in move) {
							processedVariations.add(
								`${move.nodeId.idx}-${move.nodeId.version}`,
							);
						} else {
							markProcessed(move.moves);
						}
					}
				};
				markProcessed(variationLine);

				rows.push({
					type: "variation",
					moves: variationLine,
					depth: 1,
					collapsible: variationLine.length > 5,
				});
			}
		}

		return rows;
	});

	// Helper to check if a move is current
	const isCurrentMove = (nodeId: NodeId): boolean => {
		return Boolean(
			currentNodeId.value &&
				currentNodeId.value.idx === nodeId.idx &&
				currentNodeId.value.version === nodeId.version,
		);
	};

	return {
		rootNode,
		currentNode,
		currentNodeId,
		structuredMoves,
		moveGroups,
		tableRows,
		isCurrentMove,
	};
}
