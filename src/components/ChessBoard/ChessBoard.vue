<template>
  <div
    class="chessboard"
    :style="{
      flexDirection: orientation === 'white' ? 'column-reverse' : 'column',
    }"
  >
    <div
      v-for="(row, rowIndex) in boardPosition"
      :key="rowIndex"
      class="board-row"
      :style="{
        flexDirection: orientation === 'white' ? 'row' : 'row-reverse',
      }"
    >
      <div
        v-for="(square, colIndex) in row"
        :key="colIndex"
        class="board-square"
        :class="[
          { 'is-target': square.isTarget },
          showCoordinatesClass(square),
        ]"
        :style="squareStyle(square)"
        :data-coordinates="formatCoordinates(square)"
        @click="onSquareClick(square)"
        @dragover.prevent="onDragOver(square)"
        @drop="onDrop(square)"
      >
        <AspectRatio ratio="1 / 1">
          <UseDraggable
            v-if="draggable && square.piece !== null"
            storage-type="session"
            :storage-key="`draggable-${square.row}-${square.col}`"
          >
            <div
              class="piece"
              :draggable="draggable && square.piece !== null"
              @dragstart="onDragStart($event, square)"
              @click="
                square.piece !== null
                  ? onPieceClick(square)
                  : onSquareClick(square)
              "
            >
              {{ square.piece ? pieceUnicode[square.piece] : "" }}
            </div>
          </UseDraggable>
        </AspectRatio>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { UseDraggable } from "@vueuse/components";
import { computed, onMounted, watch } from "vue";
import AspectRatio from "../AspectRatio.vue";
import { useGameStore } from "../../stores/game";
import {
  type Animation,
  type Arrows,
  type BoardStyle,
  type CoordinatesStyleType,
  type Orientation,
  type Square,
  boardStyleColorPresets,
} from "./types";
import { useChessBoard } from "./useChessBoard";
import {
  formatCoordinates,
  getSquareStyle,
  parseFenToBoard,
  pieceUnicode,
} from "./utils";

const props = defineProps({
  orientation: {
    type: String as () => Orientation,
    default: "white",
  },
  showCoordinates: {
    type: String as () => CoordinatesStyleType,
    default: "none",
  },
  draggable: {
    type: Boolean,
    default: true,
  },
  animation: {
    type: String as () => Animation,
    default: "none",
  },
  arrows: {
    type: Array as () => Arrows,
    default: () => [],
  },
  style: {
    type: Object as () => BoardStyle,
    default: () => ({
      colors: boardStyleColorPresets.blue.colors,
      squareBorderWidth: 0,
      pieceStyle: {
        unicodeColors: {
          white: "#000",
          black: "#fff",
        },
      },
    }),
  },
});

const emit = defineEmits<{
  (e: "move", move: { from: Square; to: Square }): void;
  (e: "piece-click", square: Square): void;
  (e: "square-click", square: Square): void;
}>();

const gameStore = useGameStore();

const currentPosition = computed(() => {
  return gameStore.currentPosition;
});

const {
  board,
  onDragStart,
  onDragOver,
  onDrop,
  onPieceClick,
  onSquareClick,
  draggingPiece,
  updateBoard,
} = useChessBoard(
  {
    currentPosition,
    orientation: props.orientation,
    draggable: props.draggable,
  },
  emit
);

// Watch for position changes and update the board
watch(currentPosition, () => {
  updateBoard();
});

// Watch for orientation changes and update the board
watch(() => props.orientation, () => {
  updateBoard();
});

const showCoordinatesClass = computed(() => {
  return (square: Square) => {
    if (props.showCoordinates === "none") {
      return "";
    }

    const classes = [];
    const isWhiteOrientation = props.orientation === "white";
    const firstRow = isWhiteOrientation ? 0 : 7;
    const lastRow = isWhiteOrientation ? 7 : 0;
    const firstCol = isWhiteOrientation ? 0 : 7;
    const lastCol = isWhiteOrientation ? 7 : 0;

    if (props.showCoordinates === "inside" || props.showCoordinates === "outside") {
      if (square.row === firstRow) {
        classes.push("coordinate-bottom");
      } else if (square.row === lastRow) {
        classes.push("coordinate-top");
      }
      if (square.col === firstCol) {
        classes.push("coordinate-left");
      } else if (square.col === lastCol) {
        classes.push("coordinate-right");
      }
    } else if (props.showCoordinates === "verbose") {
      classes.push(
        "coordinate-top",
        "coordinate-bottom",
        "coordinate-left",
        "coordinate-right"
      );
    }

    classes.push(`show-coordinates-${props.showCoordinates}`);
    return classes;
  };
});

const squareStyle = computed(() => {
  return (square: Square) => getSquareStyle(square, props.style);
});

const boardPosition = computed(() => {
  return board.value;
});
</script>

<style scoped>
.chessboard {
  display: flex;
  flex-direction: column-reverse;
  user-select: none;
  --coordinate-font-size: 10px;
  max-height: 100%;
  max-width: 100%;
}

.board-row {
  display: flex;
  flex: 1;
  min-height: 20px;
}

.board-square {
  flex: 1;
  position: relative;
  min-width: 20px;
  min-height: 20px;
}

.piece {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 36px;
  font-family: "Segoe UI Symbol", "Noto Sans Symbols", "DejaVu Sans", "Symbola",
    sans-serif;
}

.piece[draggable="true"] {
  cursor: grab;
  -moz-user-select: none;
  -khtml-user-select: none;
  -webkit-user-select: none;
  user-select: none;
}

.piece:active {
  cursor: grabbing;
}

.drag-image {
  font-size: 36px;
  font-family: "Segoe UI Symbol", "Noto Sans Symbols", "DejaVu Sans", "Symbola",
    sans-serif;
  pointer-events: none;
}

.is-target {
  border: 2px solid var(--p-primary-color);
}

.show-coordinates-inside::after {
  position: absolute;
  top: 0;
  left: 0;
  content: attr(data-coordinates);
  font-size: var(--coordinate-font-size);
  color: var(--p-content-color);
}

.coordinate-top.show-coordinates-outside::after,
.coordinate-bottom.show-coordinates-outside::after,
.coordinate-left.show-coordinates-outside::after,
.coordinate-right.show-coordinates-outside::after {
  position: absolute;
  content: attr(data-coordinates);
  font-size: var(--coordinate-font-size);
  color: var(--p-content-color);
}

.coordinate-top.show-coordinates-outside::after {
  top: -16px;
  left: 50%;
  transform: translateX(-50%);
}

.coordinate-bottom.show-coordinates-outside::after {
  bottom: -16px;
  left: 50%;
  transform: translateX(-50%);
}

.coordinate-left.show-coordinates-outside::after {
  left: -16px;
  top: 50%;
  transform: translateY(-50%);
}

.coordinate-right.show-coordinates-outside::after {
  right: -16px;
  top: 50%;
  transform: translateY(-50%);
}

.show-coordinates-verbose::after {
  position: absolute;
  top: 0;
  left: 0;
  content: attr(data-coordinates);
  font-size: var(--coordinate-font-size);
  color: var(--p-content-color);
}
</style>
