import { defineStore } from "pinia";
import api from "../shared/api";
import { isAPIGame, isExplorerGame } from "../shared/types";

export const useGameStore = defineStore("game", {
  state: () => ({
    games: [] as ExplorerGame[],
    selectedGame: null as APIGame | null,
    selectedGameLocation: 0,
  }),

  getters: {
    currentMove: (state): APIMove | null =>
      state.selectedGame?.moves?.[state.selectedGameLocation] ?? null,

    nextMoves: (state): APIMove[] => {
      const currentPositionId =
        state.selectedGame?.moves?.[state.selectedGameLocation]?.child_position
          ?.id;
      // Get the next move in both mainline and variations
      const nextMoves = state.selectedGame?.moves?.filter(
        (searchMove) =>
          searchMove.parent_position?.id === currentPositionId &&
          searchMove.game_move.move_number === state.selectedGameLocation + 1
      );
      return (
        nextMoves?.sort(
          (a, b) =>
            (a.game_move.variation_order ?? 0) -
            (b.game_move.variation_order ?? 0)
        ) ?? []
      );
    },

    currentPosition: (state): string =>
      state.selectedGame?.moves?.[state.selectedGameLocation]?.parent_position
        ?.fen ?? "start",

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
      try {
        console.log("Starting setSelectedGame with:", game);
        const gameId =
          isAPIGame(game) || isExplorerGame(game) ? game.game.id : null;
        // FIXME: hasGameIdChanged is undefined
        const hasGameIdChanged =
          !!this.selectedGame?.game && this.selectedGame.game.id !== gameId;
        console.log("Has game ID changed:", hasGameIdChanged);

        if (game === null || hasGameIdChanged) {
          console.log("Resetting selected game and location to null");
          this.selectedGame = null;
          this.selectedGameLocation = 0;
        }

        console.log("Calling api.setSelectedGame with gameId:", gameId);
        await api.setSelectedGame(gameId);
        console.log("api.setSelectedGame completed");

        // If the game was updated, fetch the new game
        if (hasGameIdChanged && gameId !== null) {
          console.log("Fetching new game data...");
          await this.fetchSelectedGame();
          console.log(
            "fetchSelectedGame completed, new game:",
            this.selectedGame
          );
        }
      } catch (error) {
        console.error("Error in setSelectedGame:", error);
        throw error; // Re-throw to maintain error propagation
      }
    },

    setSelectedGameLocation(location: number) {
      console.log("Setting game location:", location);
      if (location >= 0 && location < (this.selectedGame?.moves?.length ?? 0)) {
        this.selectedGameLocation = location;
        console.log(
          "New position:",
          this.selectedGame?.moves?.[location]?.parent_position.fen
        );
      }
    },

    goToNextMove() {
      console.log("Going to next move");
      if (this.canGoForward) {
        this.selectedGameLocation++;
      }
    },

    goToPreviousMove() {
      console.log("Going to previous move");
      if (this.canGoBack) {
        this.selectedGameLocation--;
      }
    },

    async makeMove(position: string, move: string) {
      await api.makeMove(position, move);
      await this.fetchSelectedGame();
      // Returns the new position
    },

    async updateGames() {
      console.log("Updating games");
      const parsedState = await api.getExplorerState();
      this.games = parsedState;
      this.setSelectedGame(parsedState[0]);
    },

    async fetchSelectedGame() {
      try {
        console.log("Starting fetchSelectedGame");
        const parsedGame = await api.getSelectedGame();
        console.log("Received parsed game:", parsedGame);
        this.selectedGame = parsedGame;
        if (parsedGame) {
          console.log("Game moves:", parsedGame.moves?.length ?? 0, "moves");
        }
      } catch (error) {
        console.error("Error in fetchSelectedGame:", error);
        throw error;
      }
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
