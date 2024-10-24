<script setup lang="ts">
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

const props = defineProps<{
  selectedGame: IGame | null;
}>();

const moveHistory = ref<string[]>([]);

const showCoordinates = ref<CoordinatesStyleType>(CoordinatesStyle.none);
const coordinatesOptions = Object.values(CoordinatesStyle);

const orientation = ref<Orientation>("black");
const toggleOrientation = () => {
  orientation.value = orientation.value === "black" ? "white" : "black";
};
</script>

<template>
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
        <div v-for="move in moveHistory" :key="move">
          {{ move }}
        </div>
      </template>

      <template #end>
        <!-- Game toolbar buttons -->
      </template>
    </Toolbar>
  </div>
  <div v-else>
    <h3>No game selected</h3>
  </div>
</template>

<style scoped>
#game-board-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  color: var(--p-primary-color);
  background-color: var(--p-content-background);
}
</style>
