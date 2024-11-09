<script setup lang="ts">
import Button from "primevue/button";
import Card from "primevue/card";
import Toolbar from "primevue/toolbar";
import { ref } from "vue";
import { useGlobalState } from "../shared/store";
import AspectRatio from "./AspectRatio.vue";
import ChessBoard from "./ChessBoard/ChessBoard.vue";
import {
  CoordinatesStyle,
  CoordinatesStyleType,
  Orientation,
} from "./ChessBoard/types";

const { selectedGame, selectedGameLocation, setSelectedGameLocation } =
  useGlobalState();

const showCoordinates = ref<CoordinatesStyleType>(CoordinatesStyle.none);

const orientation = ref<Orientation>("white");

const previousMove = () => {
  const newLocation = (selectedGameLocation.value ?? 0) - 1;
  setSelectedGameLocation(Math.max(newLocation, 0));
};

const nextMove = () => {
  const newLocation = (selectedGameLocation.value ?? 0) + 1;
  setSelectedGameLocation(
    Math.min(newLocation, (selectedGame.value?.moves?.length ?? 0) - 1),
  );
};
</script>

<template>
  <Card style="width: 100%" class="game-board">
    <template #content>
      <div v-if="selectedGame" id="game-board-container">
        <AspectRatio ratio="1 / 1">
          <ChessBoard
            initial-position="start"
            :current-position="
              selectedGame.moves?.[selectedGameLocation ?? 0]?.fen ?? 'start'
            "
            :orientation="orientation"
            :show-coordinates="showCoordinates"
            :draggable="true"
          />
        </AspectRatio>

        <Toolbar>
          <template #start>
            <!-- Debugging (display coordinates style) -->
            <!-- <Select v-model="showCoordinates" :options="coordinatesOptions" /> -->

            <!-- Button to toggle orientation -->
            <!-- <Button label="Toggle orientation" @click="toggleOrientation" /> -->

            <!-- Game navigation buttons -->
          </template>

          <template #center>
            <div class="game-board-move-number">
              <Button icon="pi pi-chevron-left" @click="previousMove" />
              <span>{{ selectedGameLocation }}</span>
              <span> / </span>
              <span>{{ selectedGame?.moves?.length }}</span>
              <Button icon="pi pi-chevron-right" @click="nextMove" />
              <span>
                {{ selectedGame?.moves?.[selectedGameLocation ?? 0]?.move_san }}
              </span>
            </div>
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
  max-width: 70vw;
  max-height: 70vh;
}

.game-board {
  /* width: 100% removed */
}

.game-board-move-number {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
</style>
