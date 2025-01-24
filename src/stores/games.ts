import { defineStore } from "pinia";
import type { ChessGame, ChessMove, ChessPosition } from "../shared/bindings";

interface ActiveGameState {
  id: string;
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
    activeGameMap: new Map<string, ActiveGameState>(),
  }),

  // All getters will be based on the index of the game
  // Meaning the game board will pass it's key and all getters will be based on that
  actions: {
    gameSpecificGetters(gameKey: string) {
      return {
        getActiveGame: () => this.activeGameMap.get(gameKey),
        getCurrentMove: () => this.activeGameMap.get(gameKey)?.currentMove,
        getNextMoves: () => this.activeGameMap.get(gameKey)?.validMoves,
      };
    },
  },
});
