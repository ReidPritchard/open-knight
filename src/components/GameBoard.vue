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
import { useGlobalState } from "../shared/store";

const { selectedGame, selectedGameLocation, setSelectedGameLocation } =
  useGlobalState();

const showCoordinates = ref<CoordinatesStyleType>(CoordinatesStyle.outside);
const coordinatesOptions = Object.values(CoordinatesStyle);

const orientation = ref<Orientation>("black");
const toggleOrientation = () => {
  orientation.value = orientation.value === "black" ? "white" : "black";
};

const previousMove = () => {
  setSelectedGameLocation((selectedGameLocation.value ?? 0) - 1);
};

const nextMove = () => {
  setSelectedGameLocation((selectedGameLocation.value ?? 0) + 1);
};
</script>

<template>
  <Card style="width: 100%" class="game-board">
    <template #content>
      <div v-if="selectedGame" id="game-board-container">
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
            <Select v-model="showCoordinates" :options="coordinatesOptions" />

            <!-- Button to toggle orientation -->
            <Button label="Toggle orientation" @click="toggleOrientation" />

            <!-- Game navigation buttons -->
          </template>

          <template #center>
            <Button label="Previous move" @click="previousMove" />
            <span>{{ selectedGameLocation }}</span>
            <span> / </span>
            <span>{{ selectedGame?.moves?.length }}</span>
            <Button label="Next move" @click="nextMove" />

            <span>
              {{ selectedGame?.moves?.[selectedGameLocation ?? 0]?.move_san }}
            </span>
          </template>

          <template #end>
            <!-- Game toolbar buttons -->
          </template>
        </Toolbar>
      </div>
    </template>
    <template #subtitle v-if="!selectedGame"> No game selected </template>
  </Card>
</template>

<style scoped>
#game-board-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  color: var(--p-card-color);
  background-color: var(--p-card-background);

  margin: auto;
  width: 100%;
  max-width: 800px;
}
</style>
