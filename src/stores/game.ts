import { defineStore } from "pinia";
import api from "../shared/api";
import type { APIGame } from "../shared/bindings/APIGame";
import type { APIMove } from "../shared/bindings/APIMove";
import type { ExplorerGame } from "../shared/bindings/ExplorerGame";

export const useGameStore = defineStore("game", {
  state: () => ({
    games: [] as ExplorerGame[],
    selectedGame: null as APIGame | null,
    selectedGameLocation: 0,
  }),

  getters: {
    currentMove: (state): APIMove | null =>
      state.selectedGame?.moves?.[state.selectedGameLocation] ?? null,

    currentPosition: (state): string =>
      state.selectedGame?.moves?.[state.selectedGameLocation].parent_position
        .fen ?? "start",

    hasGames: (state): boolean => state.games.length > 0,

    canGoForward: (state): boolean =>
      state.selectedGame !== null &&
      state.selectedGameLocation < (state.selectedGame.moves?.length ?? 0) - 1,

    canGoBack: (state): boolean =>
      state.selectedGame !== null && state.selectedGameLocation > 0,

    totalMoves: (state): number => state.selectedGame?.moves?.length ?? 0,
  },

  actions: {
    async setSelectedGame(game: ExplorerGame | APIGame | null) {
      if (game === null) {
        this.selectedGame = null;
        this.selectedGameLocation = 0;
        return;
      }

      // If it's an ExplorerGame, we need to fetch the full game data
      if (!("moves" in game)) {
        const gameResult = await api.getSelectedGame();
        this.selectedGame = gameResult;
      } else {
        this.selectedGame = game;
      }
      this.selectedGameLocation = 0;
    },

    setSelectedGameLocation(location: number) {
      if (location >= 0 && location < (this.selectedGame?.moves?.length ?? 0)) {
        this.selectedGameLocation = location;
      }
    },

    goToNextMove() {
      if (this.canGoForward) {
        this.selectedGameLocation++;
      }
    },

    goToPreviousMove() {
      if (this.canGoBack) {
        this.selectedGameLocation--;
      }
    },

    async updateGames() {
      console.log("Updating games");
      const parsedState = await api.getExplorerState();
      this.games = parsedState;
    },

    async fetchSelectedGame() {
      const parsedGame = await api.getSelectedGame();
      this.selectedGame = parsedGame;
    },

    async parsePgnText(pgnText: string) {
      await api.parsePgnText(pgnText);
      await this.updateGames();
      await this.fetchSelectedGame();
    },

    async emptyDatabase() {
      await api.emptyDatabase();
      await this.updateGames();
      this.selectedGame = null;
      this.selectedGameLocation = 0;
    },
  },
});
