import { defineStore } from "pinia";
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
    api: () => {
      console.warn("For development purposes only! Use at your own risk!");
      return api;
    },
  },
  actions: {
    async fetchExplorerGames() {
      const games = await api.games.GET.explorer();
      this.explorer.games = games;
      return games;
    },
    async importPGNGames(pgn: string) {
      await api.games.POST.importPGNGames(pgn);
      await this.fetchExplorerGames();
    },
    async resetDatabase() {
      await api.emptyDatabase();
      await this.fetchExplorerGames();
    },
    async analyzeCurrentPosition(engineName: string, boardId: number) {
      const boardGame = this.gamesStore.activeGameMap.get(boardId);
      if (!boardGame) {
        console.error("No board game found");
        return;
      }
      if (!boardGame.currentPosition) {
        console.error("No FEN found for current position");
        return;
      }
      await api.engines.POST.analyzePosition(
        engineName,
        boardGame.currentPosition.fen,
        10,
        1000
      );
    },
  },
});
