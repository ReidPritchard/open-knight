import { defineStore } from "pinia";
import { useError } from "../composables/useError";
import * as EngineService from "../services/EngineService";
import * as ImportExportService from "../services/ImportExportService";
import api from "../shared/api";
import type { ExplorerGame } from "../shared/types";
import { useEngineAnalysisStore } from "./engineAnalysis";
import { useGamesStore } from "./games";
import { useSettingsStore } from "./settings";
import { useUIStore } from "./ui";

export const useGlobalStore = defineStore("global", {
	state: () => ({
		internalGamesStore: useGamesStore(),
		internalUiStore: useUIStore(),
		internalSettingsStore: useSettingsStore(),
		internalEngineAnalysisStore: useEngineAnalysisStore(),
		explorer: {
			games: [] as ExplorerGame[],
		},
	}),
	getters: {
		gamesStore: (state) => state.internalGamesStore,
		uiStore: (state) => state.internalUiStore,
		settingsStore: (state) => state.internalSettingsStore,
		engineAnalysisStore: (state) => state.internalEngineAnalysisStore,
		explorerGames: (state) => state.explorer.games,
		activeGame: (state) => {
			const activeBoardId = state.internalUiStore.getActiveBoardId;
			const gameState = state.internalGamesStore.getBoardState(activeBoardId);
			return gameState?.game || null;
		},
		api: () => {
			console.warn("For development purposes only! Use at your own risk!");
			return api;
		},
	},
	actions: {
		async fetchExplorerGames() {
			const result = await ImportExportService.fetchExplorerGames();
			if (result.success && result.data) {
				this.explorer.games = result.data;
				return result.data;
			}
			return [];
		},
		async updateGameProperty(gameId: number, property: string, value: string) {
			const result = await ImportExportService.updateGameProperty(
				gameId,
				property,
				value,
			);
			if (result.success) {
				await this.fetchExplorerGames();
			}
		},
		async importPGNGames(pgn: string) {
			const result = await ImportExportService.importPGNGames(pgn);
			if (result.success) {
				await this.fetchExplorerGames();
			}
		},
		async resetDatabase() {
			const result = await ImportExportService.resetDatabase();
			if (result.success) {
				await this.fetchExplorerGames();
			}
		},
		async analyzeCurrentPosition(engineName: string, boardId: number) {
			const { handleGeneralError } = useError();
			const boardGame = this.gamesStore.activeGameMap.get(boardId);
			if (!boardGame) {
				handleGeneralError("UNEXPECTED", "No board game found", {
					metadata: { boardId },
				});
				return;
			}
			const currentPosition = this.gamesStore.getCurrentPosition(boardId);
			if (!currentPosition) {
				handleGeneralError("UNEXPECTED", "No FEN found for current position", {
					metadata: { boardId },
				});
				return;
			}
			await EngineService.analyzePosition(
				engineName,
				currentPosition.fen,
				10,
				1000,
			);
		},
	},
});
