<script setup lang="ts">
import Button from "primevue/button";
import Card from "primevue/card";
import Dialog from "primevue/dialog";
import Select from "primevue/select";
import Toolbar from "primevue/toolbar";
import { ref } from "vue";
import { IGame } from "../shared/types";
import AspectRatio from "./AspectRatio.vue";
import ChessBoard from "./ChessBoard/ChessBoard.vue";
import {
  CoordinatesStyle,
  CoordinatesStyleType,
  Orientation,
} from "./ChessBoard/types";
import ChessMoveTree from "./ChessMoveTree/ChessMoveTree.vue";

const props = defineProps<{
  selectedGame: IGame | null;
}>();

const showCoordinates = ref<CoordinatesStyleType>(CoordinatesStyle.outside);
const coordinatesOptions = Object.values(CoordinatesStyle);

const orientation = ref<Orientation>("black");
const toggleOrientation = () => {
  orientation.value = orientation.value === "black" ? "white" : "black";
};

const showMoveTree = ref(false);
const toggleMoveTree = () => {
  showMoveTree.value = !showMoveTree.value;
};
</script>

<template>
  <Card style="width: 100%" class="game-explorer">
    <div
      v-if="props.selectedGame"
      style="
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        padding: 1rem;
      "
      id="game-board-container"
    >
      <AspectRatio :ratio="1">
        <ChessBoard
          initial-position="start"
          :orientation="orientation"
          :show-coordinates="showCoordinates"
          :draggable="true"
        />
      </AspectRatio>

      <Toolbar>
        <template #start>
          <!-- Debugging (display coordinates style) -->
          <div>Coordinates style: {{ showCoordinates }}</div>
          <Select v-model="showCoordinates" :options="coordinatesOptions" />
          <!-- Button to toggle orientation -->
          <Button label="Toggle orientation" @click="toggleOrientation" />

          <!-- Game navigation buttons -->
        </template>

        <template #center>
          <!-- Display the move history -->
        </template>

        <template #end>
          <!-- Game toolbar buttons -->

          <!-- Toggle Move Tree -->
          <Button label="Toggle Move Tree" @click="toggleMoveTree" />
          <!-- Display the move tree in a dialog -->
          <Dialog
            v-model="showMoveTree"
            :header="`Move Tree for ${props.selectedGame?.id}`"
            :content-style="{ width: '100%' }"
            :visible="showMoveTree"
            :modal="true"
            @hide="toggleMoveTree"
          >
            <ChessMoveTree :moves="props.selectedGame?.moves || []" />
          </Dialog>
        </template>
      </Toolbar>
    </div>
    <template #subtitle> No game selected </template>
  </Card>
</template>

<style scoped>
#game-board-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  color: var(--p-card-color);
  background-color: var(--p-card-background);
}
</style>
