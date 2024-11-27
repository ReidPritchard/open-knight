import { onMounted, onUnmounted, ref, watch } from "vue";
import type { Board, Move, Orientation, Square } from "./types";
import { createBoard, parseFenToBoard } from "./utils";

export function useChessBoard(
  props: {
    currentPosition: { value: string };
    orientation: Orientation;
    draggable: boolean;
  },
  emit: {
    (event: "move", move: { from: Square; to: Square }): void;
    (event: "piece-click", square: Square): void;
    (event: "square-click", square: Square): void;
  }
) {
  const board = ref<Board>([]);
  const draggingPiece = ref<Square | null>(null);
  const dragImage = ref<HTMLDivElement | null>(null);

  const updateBoard = () => {
    if (props.currentPosition.value === "start") {
      board.value = createBoard();
    } else {
      board.value = parseFenToBoard(props.currentPosition.value);
    }
  };

  const onDragStart = (event: DragEvent, square: Square) => {
    if (!props.draggable) return;

    draggingPiece.value = square;
    event.dataTransfer?.setData("text/plain", JSON.stringify(square));

    dragImage.value = document.createElement("div");
    dragImage.value.className = "piece drag-image";
    dragImage.value.innerText = square.piece || "";
    document.body.appendChild(dragImage.value);

    dragImage.value.style.position = "absolute";
    dragImage.value.style.pointerEvents = "none";
    dragImage.value.style.zIndex = "1000";

    moveDragImage(event);
  };

  const onDragOver = (square: Square) => {
    if (draggingPiece.value && draggingPiece.value !== square) {
      event?.preventDefault();
    }
  };

  const onDrop = (toSquare: Square) => {
    if (draggingPiece.value) {
      const fromSquare = { ...draggingPiece.value };
      emit("move", { from: fromSquare, to: toSquare });
      draggingPiece.value = null;
    }

    cleanupDragImage();
  };

  const moveDragImage = (event: DragEvent) => {
    if (dragImage.value && event.pageX && event.pageY) {
      dragImage.value.style.left = `${event.pageX - 30}px`;
      dragImage.value.style.top = `${event.pageY - 30}px`;
    }
  };

  const cleanupDragImage = () => {
    if (dragImage.value) {
      document.body.removeChild(dragImage.value);
      dragImage.value = null;
    }
  };

  const onPieceClick = (square: Square) => {
    emit("piece-click", square);
  };

  const onSquareClick = (square: Square) => {
    emit("square-click", square);
  };

  // Initialize board
  onMounted(() => {
    updateBoard();
    document.addEventListener("drag", moveDragImage);
    document.addEventListener("dragend", cleanupDragImage);
  });

  // Cleanup
  onUnmounted(() => {
    document.removeEventListener("drag", moveDragImage);
    document.removeEventListener("dragend", cleanupDragImage);
    cleanupDragImage();
  });

  return {
    board,
    onDragStart,
    onDragOver,
    onDrop,
    onPieceClick,
    onSquareClick,
    draggingPiece,
    updateBoard,
  };
}
