<template>
  <div class="game-notes">
    <MeterGroup
      :value="meterItems"
      :max="totalMoves"
      label-position="start"
    ></MeterGroup>
    <Inplace class="inline-notes-editor">
      <template #display>
        <p>{{ currentMove?.game_move.annotation }}</p>
        <Button icon="pi pi-pencil" />
      </template>
      <template #content="{ closeCallback }">
        <InputText v-model="moveNotes" autofocus @keyup.enter="closeCallback" />
        <Button
          icon="pi pi-check"
          text
          severity="success"
          @click="closeCallback"
        />
      </template>
    </Inplace>
  </div>
</template>

<script setup lang="ts">
import Inplace from "primevue/inplace";
import MeterGroup from "primevue/metergroup";
import { computed } from "vue";
import { useGlobalStore } from "../stores";

const { game } = useGlobalStore();

const moveNotes = computed({
  get: () => {
    return currentMove.value?.game_move.annotation ?? "";
  },
  set: (value) => {
    updateMoveNotes(value);
  },
});

const updateMoveNotes = (value: string) => {
  if (currentMove.value) {
    currentMove.value.game_move.annotation = value;
  }
};

const totalMoves = computed(() => {
  return game.selectedGame?.moves.length ?? 0;
});

const currentMove = computed(() => {
  return game.selectedGame?.moves[game.selectedGameLocation ?? 0];
});

const meterItems = computed(() => {
  return [
    {
      value: game.selectedGameLocation ?? 0,
      color: "var(--p-primary-color)",
      label: "Game Position",
      icon: "pi pi-gamepad",
    },
  ];
});
</script>

<style scoped>
.game-notes {
  background-color: var(--surface-ground);
  padding: 1rem;

  max-width: 400px;

  overflow: scroll;
}

.game-notes > * {
  width: 100%;
}

.inline-notes-editor > * {
  display: flex;
  flex-direction: row;

  align-items: center;
  justify-content: space-between;

  gap: 0.5rem;
}
.inline-notes-editor > p {
  margin: 0;
  padding: 0;

  line-break: anywhere;

  flex-grow: 1;
}
</style>
