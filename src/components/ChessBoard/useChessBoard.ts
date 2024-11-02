import { onMounted, ref } from "vue";
import { Board, Move, Orientation, Square } from "./types";
import { createBoard } from "./utils";

export function useChessBoard(
  props: {
    initialPosition: string;
    currentPosition: string;
    orientation: Orientation;
    draggable: boolean;
  },
  emit: (
    event: "move" | "piece-click" | "square-click",
    ...args: any[]
  ) => void,
) {
  const board = ref<Board>([]);
  const draggingPiece = ref<Square | null>(null);
  const dragImage = ref<HTMLDivElement | null>(null);

  const initBoard = () => {
    if (props.initialPosition === "start") {
      board.value = createBoard();
    }
  };

  const onDragStart = (event: DragEvent, square: Square) => {
    console.log("drag-start", square);
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
    console.log("drag-over", square);
    if (draggingPiece.value && draggingPiece.value !== square) {
      event?.preventDefault();
    }
  };

  const onDrop = (square: Square) => {
    console.log("drag-drop", square);
    if (draggingPiece.value) {
      board.value[square.row][square.col].piece = draggingPiece.value.piece;
      board.value[draggingPiece.value.row][draggingPiece.value.col].piece =
        null;

      const move: Move = {
        from: { row: draggingPiece.value.row, col: draggingPiece.value.col },
        to: { row: square.row, col: square.col },
        piece: draggingPiece.value.piece!,
      };
      emit("move", move);

      draggingPiece.value = null;
    }
    if (dragImage.value) {
      document.body.removeChild(dragImage.value);
      dragImage.value = null;
    }
  };

  const moveDragImage = (event: DragEvent) => {
    if (dragImage.value) {
      dragImage.value.style.left = `${event.pageX - 30}px`;
      dragImage.value.style.top = `${event.pageY - 30}px`;
    }
  };

  const onDragEnd = () => {
    console.log("drag-end");
    if (dragImage.value) {
      document.body.removeChild(dragImage.value);
      dragImage.value = null;
    }
  };

  const onPieceClick = (square: Square) => {
    console.log("piece-click", square);
    emit("piece-click", square);
  };

  const onSquareClick = (square: Square) => {
    console.log("square-click", square);
    emit("square-click", square);
  };

  onMounted(() => {
    initBoard();
    document.addEventListener("drag", moveDragImage);
    document.addEventListener("dragend", onDragEnd);
  });

  return {
    board,
    onDragStart,
    onDragOver,
    onDrop,
    onPieceClick,
    onSquareClick,
    draggingPiece,
  };
}
