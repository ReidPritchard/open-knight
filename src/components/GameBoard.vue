<script setup lang="ts">
import { TheChessboard, type BoardApi } from 'vue3-chessboard';
import 'vue3-chessboard/style.css';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import { watch, ref } from 'vue';
import { IGameBoardGame } from '../shared/types';

const props = defineProps<{
    selectedGame: IGameBoardGame | null;
}>();

const boardAPI = ref<BoardApi | undefined>(undefined);
const moveHistory = ref<string[]>([]);

watch(() => props.selectedGame, (game) => {
    console.log("Game changed:", game);
    boardAPI.value?.loadPgn(game?.pgn ?? '');
}, { immediate: true });
</script>

<template>
    <div v-if="selectedGame" style="display: flex; flex-direction: column; gap: 1rem;">
        <TheChessboard @board-created="(api) => (boardAPI = api)" />

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
            </template>

            <template #end>
                <!-- Game toolbar buttons -->
            </template>
        </Toolbar>
    </div>
    <div v-else>

    </div>
</template>