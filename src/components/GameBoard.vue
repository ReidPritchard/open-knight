<script setup lang="ts">
import { TheChessboard, type BoardApi } from 'vue3-chessboard';
import 'vue3-chessboard/style.css';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import { watch, ref, onMounted } from 'vue';
import { GameBoardGame } from '../App.vue';
import { invoke } from '@tauri-apps/api/core';
import { apiSelectedGameToGame } from '../shared/api-conversions';

const selectedGame = ref<GameBoardGame | null>(null);
const boardAPI = ref<BoardApi | undefined>(undefined);
const moveHistory = ref<string[]>([]);


watch(() => selectedGame.value, (game) => {
    console.log("Game changed:", game);

    boardAPI.value?.loadPgn(game?.pgn ?? '');
}, { immediate: true });

async function getSelectedGame() {
    const selectedGameApiResponse: string = await invoke("get_selected_game");
    selectedGame.value = apiSelectedGameToGame(selectedGameApiResponse);
}

onMounted(async () => {
    await getSelectedGame();
});

</script>

<template>
    <TheChessboard v-if="selectedGame" @board-created="(api) => (boardAPI = api)" />
    <p v-else>No game selected</p>

    <Toolbar>
        <template #start>
            <!-- Game navigation buttons -->
            <Button icon="pi pi-step-backward" @click="boardAPI?.viewPrevious()" />
            <Button icon="pi pi-backward" @click="boardAPI?.undoLastMove()" />
            <Button icon="pi pi-step-forward" @click="boardAPI?.viewNext()" />
            <Button icon="pi pi-forward" @click="boardAPI?.viewPrevious()" />
        </template>

        <template #center>
            <!-- Display the move history -->
            <div v-for="move in moveHistory" :key="move">
                {{ move }}
            </div>
            <!-- Clear the current game -->
            <Button icon="pi pi-times" @click="selectedGame = null" />
        </template>

        <template #end>
            <!-- Game toolbar buttons -->
        </template>
    </Toolbar>
</template>
