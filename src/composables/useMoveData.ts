import { computed, type ComputedRef } from "vue";
import type { ChessMoveTree, ChessTreeNode } from "../shared/bindings";
import type { MoveData, MoveGroup, TableRow, TableVariationRow, VariationMove, NodeId } from "../shared/types";

export function useMoveData(moveTree: ComputedRef<ChessMoveTree>) {
	const rootNode = computed((): ChessTreeNode | null => {
		if (!moveTree.value.root_id) return null;
		return moveTree.value.nodes[moveTree.value.root_id.idx]?.value || null;
	});

	const currentNodeId = computed(() => moveTree.value.current_node_id);

	const currentNode = computed((): ChessTreeNode | null => {
		if (!currentNodeId.value) return null;
		return moveTree.value.nodes[currentNodeId.value.idx]?.value || null;
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
			const nodeWrapper = moveTree.value.nodes.find((n) => n.value === node);
			if (!nodeWrapper) return [];

			const nodeId: NodeId = {
				idx: moveTree.value.nodes.indexOf(nodeWrapper),
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
				.map((childId) => moveTree.value.nodes[childId.idx]?.value)
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
							v.plyNumber ===
								currentChain[currentChain.length - 1].plyNumber + 1
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

		// Group main moves by move number (1 move = 2 plys)
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

		// Handle variations within variations (nested variations)
		// Nested variations are displayed as indented rows after their parent move.
		// Variation rows that span multiple lines can be marked as collapsible.

		// Create table rows with variations
		for (const [number, moves] of Object.entries(movesByNumber)) {
			const moveNumber = Number.parseInt(number);
			rows.push({
				type: "move",
				number: moveNumber,
				white: moves.white || undefined,
				black: moves.black || undefined,
			});

			// Add variations that branch from this move
			const moveRowVariations = variations.filter(
				(v) => v.parentMoveNumber === moveNumber,
			);

			if (moveRowVariations.length > 0) {
				// Helper function to collect complete variation chains with nested variations
				const collectVariationChain = (
					startMove: MoveData,
					depth: number = 1
				): VariationMove[] => {
					const chain: VariationMove[] = [startMove];

					// Traverse children to collect the complete variation line
					let currentNode = startMove.node;
					while (
						currentNode.children_ids &&
						currentNode.children_ids.length > 0
					) {
						const childIds = currentNode.children_ids;
						
						// Follow the first child to continue the main variation line
						const firstChildId = childIds[0];
						const firstChildWrapper = moveTree.value.nodes[firstChildId.idx];

						if (!firstChildWrapper?.value) break;

						const firstChild = firstChildWrapper.value;
						if (!firstChild.game_move) break;

						// Create MoveData for the first child move
						const firstChildMoveData: MoveData = {
							nodeId: { idx: firstChildId.idx, version: firstChildWrapper.version },
							node: firstChild,
							move: firstChild.game_move,
							san: firstChild.game_move.san,
							plyNumber: firstChild.game_move.ply_number,
							moveNumber: Math.ceil(firstChild.game_move.ply_number / 2),
							showNumber: firstChild.game_move.ply_number % 2 === 1,
							isWhite: firstChild.game_move.ply_number % 2 === 1,
							isMainLine: false,
							isVariation: true,
							depth: startMove.depth,
							parentMoveNumber: startMove.parentMoveNumber,
						};

						chain.push(firstChildMoveData);

						// Handle additional children as nested variations
						if (childIds.length > 1) {
							for (let i = 1; i < childIds.length; i++) {
								const nestedChildId = childIds[i];
								const nestedChildWrapper = moveTree.value.nodes[nestedChildId.idx];
								
								if (!nestedChildWrapper?.value?.game_move) continue;

								const nestedChild = nestedChildWrapper.value;
								if (!nestedChild.game_move) continue;
								
								const nestedMoveData: MoveData = {
									nodeId: { idx: nestedChildId.idx, version: nestedChildWrapper.version },
									node: nestedChild,
									move: nestedChild.game_move,
									san: nestedChild.game_move.san,
									plyNumber: nestedChild.game_move.ply_number,
									moveNumber: Math.ceil(nestedChild.game_move.ply_number / 2),
									showNumber: nestedChild.game_move.ply_number % 2 === 1,
									isWhite: nestedChild.game_move.ply_number % 2 === 1,
									isMainLine: false,
									isVariation: true,
									depth: depth + 1,
									parentMoveNumber: firstChildMoveData.moveNumber,
								};

								// Recursively collect the nested variation chain
								const nestedChain = collectVariationChain(nestedMoveData, depth + 1);
								
								// Create nested variation row
								const nestedVariation: TableVariationRow = {
									type: "variation",
									moves: nestedChain,
									depth: depth + 1,
									collapsible: nestedChain.length > 3, // Mark as collapsible if more than 3 moves
								};

								chain.push(nestedVariation);
							}
						}

						currentNode = firstChild;
					}

					return chain;
				};

				// Group variations into complete chains
				const processedNodes = new Set<string>();

				// For each variation starting move, collect its complete chain
				for (const v of moveRowVariations) {
					const nodeKey = `${v.nodeId.idx}-${v.nodeId.version}`;
					if (processedNodes.has(nodeKey)) continue;

					const completeChain = collectVariationChain(v, 1);

					// Mark all nodes in this chain as processed (recursively)
					const markProcessed = (moves: VariationMove[]) => {
						for (const move of moves) {
							if ('nodeId' in move) {
								// It's a MoveData
								const key = `${move.nodeId.idx}-${move.nodeId.version}`;
								processedNodes.add(key);
							} else {
								// It's a nested TableVariationRow
								markProcessed(move.moves);
							}
						}
					};
					markProcessed(completeChain);

					rows.push({
						type: "variation",
						moves: completeChain,
						depth: 1,
						collapsible: completeChain.length > 5, // Mark as collapsible if more than 5 moves/sub-variations
					});
				}
			}
		}

		return rows;
	});

	const isCurrentMove = (nodeId: NodeId): boolean => {
		return (
			currentNodeId.value !== undefined &&
			currentNodeId.value.idx === nodeId.idx &&
			currentNodeId.value.version === nodeId.version
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
