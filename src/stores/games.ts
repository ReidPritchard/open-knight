import { defineStore } from "pinia";
import api from "../shared/api";
import type { ChessGame, ChessMove, ChessPosition } from "../shared/bindings";

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

  getters: {
    getBoardState: (state) => (boardId: number) =>
      state.activeGameMap.get(boardId),
  },

  actions: {
    async openGame(gameId: number, boardId: number) {
      // Check if game is already open
      if (this.activeGameMap.has(boardId)) {
        return;
      }

      // Open game
      const game = await api.games.GET.game(gameId);
      const initialPosition = game.moves[0]?.position;
      const validMoves = game.moves[0]?.next_move
        ? [game.moves[0].next_move]
        : null;

      console.log("Game:", game);
      console.log("Moves:", game.moves.length, game.moves);
      console.log("Initial position:", initialPosition);

      const newGameState: ActiveGameState = {
        id: gameId,
        game: game,

        currentMoveIndex: 0,
        currentMove: game.moves[0],
        currentPosition: initialPosition,
        validMoves: validMoves,

        inProgress: false,
        userIsPlaying: null,

        hideEvaluationBar: false,
        hideBestMove: false,
        hideThreats: false,
      };

      this.activeGameMap.set(boardId, newGameState);

      return newGameState;
    },

    async closeGame(boardId: number) {
      this.activeGameMap.delete(boardId);
    },
    async nextMove(boardId: number) {
      const game = this.activeGameMap.get(boardId);
      if (game) {
        const currentMove = game.currentMove;
        if (currentMove?.next_move) {
          game.currentMove = currentMove.next_move;
          game.validMoves = currentMove.next_move.next_move
            ? [currentMove.next_move.next_move]
            : null;
          game.currentMoveIndex++;
        } else {
          const nextMove = game.game.moves[game.currentMoveIndex + 1];
          if (nextMove) {
            game.currentMove = nextMove;
            game.currentMoveIndex++;
            game.validMoves = nextMove.next_move ? [nextMove.next_move] : null;
          }
        }
      }
    },
    async previousMove(boardId: number) {
      const game = this.activeGameMap.get(boardId);
      if (game) {
        if (game.currentMoveIndex > 0) {
          game.currentMoveIndex--;
          const prevMove = game.game.moves[game.currentMoveIndex];
          game.currentMove = prevMove;
          game.validMoves = prevMove.next_move ? [prevMove.next_move] : null;
        }
      }
    },

    async jumpToMove(boardId: number, moveId: number) {
      const game = this.activeGameMap.get(boardId);
      if (game) {
        // Find the move in the game
        const move = game.game.moves.find((m) => m.id === moveId);
        if (move) {
          game.currentMoveIndex = move.ply_number;
          game.currentMove = move;
          game.currentPosition = move.position;
          game.validMoves = move.next_move ? [move.next_move] : null;
        }
      }
    },
  },
});
