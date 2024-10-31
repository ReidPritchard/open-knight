<template>
  <div
    class="chessboard"
    :style="{
      flexDirection: orientation === 'white' ? 'column-reverse' : 'column',
    }"
    @mousedown.prevent
  >
    <div
      v-for="(row, rowIndex) in board"
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
          getSquareColor(square.row, square.col),
          { 'is-target': square.isTarget },
          showCoordinatesClass(square),
        ]"
        :data-coordinates="formatCoordinates(square)"
        @click="onSquareClick(square)"
        @dragover.prevent="onDragOver(square)"
        @drop="onDrop(square)"
      >
        <AspectRatio :ratio="1">
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

<script lang="ts">
import { UseDraggable } from "@vueuse/components";
import { defineComponent, PropType } from "vue";
import AspectRatio from "../AspectRatio.vue";
import {
  Animation,
  Arrows,
  CoordinatesStyleType,
  Orientation,
  Square,
} from "./types";
import { useChessBoard } from "./useChessBoard";
import { formatCoordinates, getSquareColor, pieceUnicode } from "./utils";

export default defineComponent({
  name: "Chessboard",
  props: {
    initialPosition: {
      type: String,
      default: "start",
    },
    orientation: {
      type: String as PropType<Orientation>,
      default: "white",
    },
    showCoordinates: {
      type: String as PropType<CoordinatesStyleType>,
      default: "none",
    },
    draggable: {
      type: Boolean,
      default: true,
    },
    animation: {
      type: String as PropType<Animation>,
      default: "none",
    },
    arrows: {
      type: Array as PropType<Arrows>,
      default: () => [],
    },
  },
  emits: ["move", "piece-click", "square-click"],
  setup(props, { emit }) {
    const {
      board,
      onDragStart,
      onDragOver,
      onDrop,
      onPieceClick,
      onSquareClick,
      draggingPiece,
    } = useChessBoard(props, emit);

    return {
      board,
      getSquareColor,
      pieceUnicode,
      onDragStart,
      onDragOver,
      onDrop,
      onPieceClick,
      onSquareClick,
      draggingPiece,
      formatCoordinates,
    };
  },
  computed: {
    showCoordinatesClass() {
      return (square: Square) => {
        if (this.showCoordinates === "none") {
          return "";
        }

        const classes = [];

        const isWhiteOrientation = this.orientation === "white";

        const firstRow = isWhiteOrientation ? 0 : 7;
        const lastRow = isWhiteOrientation ? 7 : 0;
        const firstCol = isWhiteOrientation ? 0 : 7;
        const lastCol = isWhiteOrientation ? 7 : 0;

        if (
          this.showCoordinates === "inside" ||
          this.showCoordinates === "outside"
        ) {
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
        } else if (this.showCoordinates === "verbose") {
          classes.push(
            "coordinate-top",
            "coordinate-bottom",
            "coordinate-left",
            "coordinate-right",
          );
        }

        classes.push(`show-coordinates-${this.showCoordinates}`);

        return classes;
      };
    },
  },
  components: {
    AspectRatio,
    UseDraggable,
  },
});
</script>

<style scoped>
.chessboard {
  display: flex;
  flex-direction: column-reverse;

  width: 100%;
  height: 100%;

  border: 2px solid var(--p-content-color);

  user-select: none;

  --coordinate-font-size: 10px;
}

.board-row {
  display: flex;
  flex: 1;
}

.board-square {
  flex: 1;
  position: relative;
}

.light-square {
  background-color: #f0d9b5;
}

.dark-square {
  background-color: #b58863;
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

  /* Prevent text selection */
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
  border: 2px solid red;
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
