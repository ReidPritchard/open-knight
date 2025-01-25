import { defineStore } from "pinia";
import type { ChessGame, ChessMove, ChessPosition } from "../shared/bindings";
import api from "../shared/api";

interface ActiveGameState {
  id: number;
  game: ChessGame;

  currentMoveIndex: number;
  currentMove: ChessMove | null;
  currentPosition: ChessPosition | null;
  validMoves: ChessMove[] | null;

  inProgress: boolean;
  userIsPlaying: "white" | "black" | null;

  hideEvaluationBar: boolean;
  hideBestMove: boolean;
  hideThreats: boolean;
}

/**
 * A store for managing the states of ALL open games
 */
export const useGamesStore = defineStore("games", {
  state: () => ({
    activeGameMap: new Map<number, ActiveGameState>(),
  }),

  // All getters will be based on the index of the game
  // Meaning the game board will pass it's key and all getters will be based on that
  getters: {
    getBoardStore: (state) => (boardId: number) => ({
      getActiveGame: () => state.activeGameMap.get(boardId),
      getCurrentMove: () => state.activeGameMap.get(boardId)?.currentMove,
      getNextMoves: () => state.activeGameMap.get(boardId)?.validMoves,
    }),
  },

  actions: {
    async openGame(gameId: number, boardId: number) {
      // Check if game is already open
      if (this.activeGameMap.has(boardId)) {
        return;
      }

      // Open game
      const game = await api.games.GET.game(gameId);
      const newGameState: ActiveGameState = {
        id: gameId,
        game: game,

        currentMoveIndex: 0,
        currentMove: null,
        currentPosition: null,
        validMoves: null,

        inProgress: false,
        userIsPlaying: null,

        hideEvaluationBar: false,
        hideBestMove: false,
        hideThreats: false,
      };

      this.activeGameMap.set(boardId, newGameState);

      return newGameState;
    },
  },
});
