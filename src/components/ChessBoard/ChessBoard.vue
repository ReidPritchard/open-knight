<template>

	<div
		class="relative"
		:style="`width: ${squareSize * 8}px; height: ${squareSize * 8}px;`"
		ref="boardEl"
	>

		<!-- Chess Board Grid -->

		<div
			class="grid grid-rows-8 grid-flow-col transition-all duration-100 w-full h-full"
			:class="{ 'rotate-180': isFlipped }"
		>

			<div
				v-for="row in 8"
				:key="row - 1"
				class="flex"
			>

				<ChessBoardSquare
					v-for="col in 8"
					:key="col - 1"
					:row="row - 1"
					:col="col - 1"
					:square-size="squareSize"
					:piece="getPieceAt(row - 1, col - 1)"
					:piece-image="getPieceImagePath(getPieceAt(row - 1, col - 1) ?? '')"
					:can-move="canMovePiece(row - 1, col - 1)"
					:is-selected="isSquareSelected(row - 1, col - 1)"
					:is-valid-move="isValidMoveTarget(row - 1, col - 1)"
					:is-highlighted="isPartOfCurrentMove(row - 1, col - 1)"
					:is-board-flipped="isFlipped"
					:board-theme="theme"
					:class="{ 'rotate-180': isFlipped, 'rotate-0': !isFlipped }"
					@drop="handleDrop(row - 1, col - 1)"
					@click="handleSquareClick($event, row - 1, col - 1)"
					@contextmenu.prevent="
						handleSquareContextMenu($event, row - 1, col - 1)
					"
					@drag-start="handleDragStart(row - 1, col - 1)"
				/>

			</div>

		</div>

		<!-- Annotation Arrow -->

		<div
			v-if="arrowCoordinates"
			class="absolute inset-0 pointer-events-none"
			@contextmenu.prevent
		>

			<AnnotationArrow
				:from="arrowCoordinates.from"
				:to="arrowCoordinates.to"
				:options="{ color: 'yellow', size: 5 }"
			/>

		</div>

		<!-- Temporary Annotation Arrows (user-created, not saved) -->

		<template v-if="temporaryAnnotations.length > 0">

			<div
				v-for="(annotation, index) in temporaryAnnotations"
				:key="`temp-${index}`"
				class="absolute inset-0 pointer-events-none"
				@contextmenu.prevent
			>

				<AnnotationArrow
					:from="annotation.coordinates.from"
					:to="annotation.coordinates.to"
					:options="annotation.options"
				/>

			</div>

		</template>

		<!-- Active Arrow Being Drawn (real-time feedback during drag) -->

		<div
			v-if="activeArrow"
			class="absolute inset-0"
			@contextmenu.prevent
		>

			<AnnotationArrow
				:from="activeArrow.from"
				:to="activeArrow.to"
				:options="activeArrow.options"
			/>

		</div>

	</div>

	<!-- Move Navigation -->

	<div class="flex flex-row items-center justify-center mt-4">

		<!-- Rotate Board -->

		<button
			class="btn btn-sm ml-4"
			@click="emit('rotate-board')"
		>

			<PhArrowsClockwise size="16" />

		</button>

		<!-- Resize Board (drag to resize) -->

		<div
			class="ml-4 cursor-ew-resize"
			@mousedown="startResize"
		>

			<PhArrowsOutLineHorizontal size="16" />

		</div>

		<!-- Clear Annotations -->

		<button
			v-if="temporaryAnnotations.length > 0"
			class="btn btn-sm ml-4"
			@click="clearAnnotations"
			title="Clear annotations (Escape)"
		>

			<PhX size="16" />

		</button>

	</div>

</template>

<script setup lang="ts">
import {
	PhArrowsClockwise,
	PhArrowsOutLineHorizontal,
	PhX,
} from "@phosphor-icons/vue";
import { computed, onMounted, onUnmounted, ref } from "vue";
import type { BoardTheme } from "../../shared/themes";
import type {
	ChessMove,
	ChessPosition,
	LegalMove,
} from "../../shared/bindings";
import AnnotationArrow from "../AnnotationArrow/AnnotationArrow.vue";
import ChessBoardSquare from "./ChessBoardSquare.vue";
import {
	boardToAlgebraic,
	calculateSquareCenter,
	getPieceImagePath,
	isAnnotationClick,
	parseFen,
	parseUciMove,
	type BoardState,
} from "./utils";

const props = defineProps<{
	boardId: number;
	position: ChessPosition | null;
	move: ChessMove | null;
	validMoves: LegalMove[] | null;
	theme: BoardTheme;
	showCoordinates: boolean;
	showLegalMoves: boolean;
	isFlipped: boolean;
}>();

const emit = defineEmits<{
	(e: "make-move", move: LegalMove): void;
	(e: "rotate-board"): void;
	(e: "resize-board", size: number): void;
	(e: "select-square", square: string): void;
	(e: "add-annotation", annotation: any): void;
	(e: "clear-annotations"): void;
}>();

const boardEl = ref<HTMLElement | null>(null);
const squareSize = ref(64);

const pieces = computed((): BoardState => {
	if (!props.position) return Array(8).fill(Array(8).fill(""));
	return parseFen(props.position.fen);
});

const getPieceAt = (row: number, col: number) => {
	return pieces.value[row]?.[col];
};

const selectedSquare = ref<string | null>(null);
const isSquareSelected = (row: number, col: number) => {
	return selectedSquare.value === boardToAlgebraic(row, col);
};

const handleSquareClick = (event: MouseEvent, row: number, col: number) => {
	if (isAnnotationClick(event)) return;
	const clickedAN = boardToAlgebraic(row, col);

	if (selectedSquare.value) {
		const fromAN = selectedSquare.value;
		const toAN = clickedAN;
		const move = props.validMoves?.find(
			(m) => m.uci === `${fromAN}${toAN}` || m.uci === `${fromAN}${toAN}q`, // promotions
		);
		if (move) {
			emit("make-move", move);
			selectedSquare.value = null;
		} else {
			selectedSquare.value = clickedAN;
		}
	} else {
		selectedSquare.value = clickedAN;
	}
	emit("select-square", clickedAN);
};

const canMovePiece = (row: number, col: number) => {
	if (!props.showLegalMoves) return false;
	const an = boardToAlgebraic(row, col);
	const piece = getPieceAt(row, col);
	if (!piece) return false;
	return props.validMoves?.some((move) => move.uci.startsWith(an)) ?? false;
};

const isValidMoveTarget = (row: number, col: number) => {
	if (!props.showLegalMoves || !selectedSquare.value) return false;
	const fromAN = selectedSquare.value;
	const toAN = boardToAlgebraic(row, col);
	return (
		props.validMoves?.some(
			(move) =>
				move.uci === `${fromAN}${toAN}` || move.uci === `${fromAN}${toAN}q`,
		) ?? false
	);
};

const currentMoveSquares = computed(() => {
	if (!props.move) return { from: null, to: null };
	return parseUciMove(props.move.uci);
});

const isPartOfCurrentMove = (row: number, col: number) => {
	const an = boardToAlgebraic(row, col);
	return (
		an === currentMoveSquares.value.from || an === currentMoveSquares.value.to
	);
};

// Drag and drop logic
const draggedPiece = ref<string | null>(null);

const handleDragStart = (row: number, col: number) => {
	const an = boardToAlgebraic(row, col);
	const piece = getPieceAt(row, col);
	if (piece) {
		draggedPiece.value = piece;
		selectedSquare.value = an;
	}
};

const handleDrop = (row: number, col: number) => {
	if (draggedPiece.value && selectedSquare.value) {
		const fromAN = selectedSquare.value;
		const toAN = boardToAlgebraic(row, col);
		const move = props.validMoves?.find(
			(m) => m.uci === `${fromAN}${toAN}` || m.uci === `${fromAN}${toAN}q`,
		);
		if (move) {
			emit("make-move", move);
		}
	}
	draggedPiece.value = null;
	selectedSquare.value = null;
};

// =================================================================================================
// Board Resizing
// =================================================================================================

const startResize = (event: MouseEvent) => {
	event.preventDefault();
	document.addEventListener("mousemove", doResize);
	document.addEventListener("mouseup", stopResize);
};
const doResize = (event: MouseEvent) => {
	if (boardEl.value) {
		const newWidth = boardEl.value.offsetWidth + event.movementX * 2;
		squareSize.value = newWidth / 8;
	}
};
const stopResize = () => {
	document.removeEventListener("mousemove", doResize);
	document.removeEventListener("mouseup", stopResize);
	emit("resize-board", squareSize.value);
};

// =================================================================================================
// Annotations
// =================================================================================================

interface ArrowAnnotation {
	coordinates: { from: { x: number; y: number }; to: { x: number; y: number } };
	options: { color: string; size: number };
}

const arrowCoordinates = ref<{
	from: { x: number; y: number };
	to: { x: number; y: number };
} | null>(null);

const temporaryAnnotations = ref<ArrowAnnotation[]>([]);
const activeArrow = ref<{
	from: { x: number; y: number };
	to: { x: number; y: number };
	options: { color: string; size: number };
} | null>(null);
const drawingArrow = ref(false);

const getAnnotationColor = (event: MouseEvent) => {
	if (event.shiftKey) return "green";
	if (event.ctrlKey || event.metaKey) return "red";
	if (event.altKey) return "blue";
	return "yellow";
};

const handleSquareContextMenu = (
	event: MouseEvent,
	row: number,
	col: number,
) => {
	const from = calculateSquareCenter(
		col,
		row,
		squareSize.value,
		props.isFlipped,
	);
	const color = getAnnotationColor(event);

	drawingArrow.value = true;
	activeArrow.value = {
		from,
		to: from, // Initially, to is same as from
		options: { color, size: 3 },
	};

	const onMouseMove = (moveEvent: MouseEvent) => {
		if (drawingArrow.value && activeArrow.value && boardEl.value) {
			const rect = boardEl.value.getBoundingClientRect();
			const x = moveEvent.clientX - rect.left;
			const y = moveEvent.clientY - rect.top;
			activeArrow.value.to = { x, y };
		}
	};

	const onMouseUp = (upEvent: MouseEvent) => {
		drawingArrow.value = false;
		if (activeArrow.value) {
			const rect = boardEl.value!.getBoundingClientRect();
			const toRow = Math.floor((upEvent.clientY - rect.top) / squareSize.value);
			const toCol = Math.floor(
				(upEvent.clientX - rect.left) / squareSize.value,
			);
			const to = calculateSquareCenter(toCol, toRow, squareSize.value, false);

			// Only add annotation if it's not the same as the from square
			if (
				to.x !== activeArrow.value.from.x ||
				to.y !== activeArrow.value.from.y
			) {
				temporaryAnnotations.value.push({
					coordinates: { from: activeArrow.value.from, to },
					options: activeArrow.value.options,
				});
			}
			emit("add-annotation", {
				from: boardToAlgebraic(row, col),
				to: boardToAlgebraic(toRow, toCol),
				color: activeArrow.value.options.color,
			});
		}
		activeArrow.value = null;
		document.removeEventListener("mousemove", onMouseMove);
		document.removeEventListener("mouseup", onMouseUp);
	};

	document.addEventListener("mousemove", onMouseMove);
	document.addEventListener("mouseup", onMouseUp);
};

const clearAnnotations = () => {
	temporaryAnnotations.value = [];
	emit("clear-annotations");
};

// Keyboard shortcut to clear annotations
const handleKeyDown = (event: KeyboardEvent) => {
	if (event.key === "Escape") {
		clearAnnotations();
	}
};

onMounted(() => {
	document.addEventListener("keydown", handleKeyDown);
	if (boardEl.value) {
		squareSize.value = boardEl.value.offsetWidth / 8;
	}
});

onUnmounted(() => {
	document.removeEventListener("keydown", handleKeyDown);
});
</script>

<!-- 
By jurgenwesterhof (adapted from work of Cburnett) - 
http://commons.wikimedia.org/wiki/Template:SVG_chess_pieces
CC BY-SA 3.0
Link: https://commons.wikimedia.org/w/index.php?curid=35634436
-->

