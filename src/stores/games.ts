import { defineStore } from "pinia";
import api from "../shared/api";
import type {
  ChessGame,
  ChessMove,
  ChessPosition,
  LegalMove,
} from "../shared/bindings";

interface ActiveGameState {
  id: number;
  game: ChessGame;

  currentMoveIndex: number;
  currentMove: ChessMove | null;
  currentPosition: ChessPosition | null;
  validMoves: LegalMove[] | null;

  inProgress: boolean;
  userIsPlaying: "white" | "black" | null;

  hideEvaluationBar: boolean;
  hideBestMove: boolean;
  hideThreats: boolean;
}

const getValidMoves = async (position?: string) => {
  if (!position) return null;
  try {
    return await api.moves.GET.validMoves(position);
  } catch (error) {
    console.error("Failed to fetch valid moves:", error);
    return null;
  }
};

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
      // FIXME: Handle variable starting positions
      const initialPosition: ChessPosition = {
        id: 0,
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        evaluations: [],
      };
      const validMoves = await getValidMoves(initialPosition?.fen);

      console.log("Game:", game);

      const newGameState: ActiveGameState = {
        id: gameId,
        game: game,

        currentMoveIndex: -1,
        currentMove: null,
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
      // Get the game for the board
      const game = this.activeGameMap.get(boardId);
      if (!game) return;

      // Get the game's current move
      const currentMoveIndex = game.currentMoveIndex;
      const currentMove = game.currentMove;

      // If the current move index is -1, it's the starting position
      // So we need to set the current move to the first move
      if (currentMoveIndex === -1) {
        game.currentMove = game.game.moves[0];
        game.currentMoveIndex = 0;
      } else if (currentMove?.next_move) {
        // If the current move has a next move, go to it
        // TODO: Handle variations!!
        game.currentMove = currentMove.next_move;
        game.currentMoveIndex++;
      } else {
        // If the move doesn't have a next move, assume we are at the end of the game
        // and cannot go forward
        console.log("No next move", currentMove);
        return;
      }

      // Update the game's current position and valid moves
      game.currentPosition = game.currentMove?.position;
      game.validMoves = await getValidMoves(game.currentPosition?.fen);
    },

    async previousMove(boardId: number) {
      const game = this.activeGameMap.get(boardId);
      if (game) {
        if (game.currentMoveIndex > 0) {
          game.currentMoveIndex--;
          const prevMove = game.game.moves[game.currentMoveIndex];
          game.currentMove = prevMove;
          game.validMoves = await getValidMoves(prevMove.position?.fen);
        } else {
          // If the current move index is 0, move to the starting position
          game.currentMove = null;
          game.currentMoveIndex = -1;
          game.currentPosition = {
            id: 0,
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            evaluations: [],
          };
          game.validMoves = await getValidMoves(game.currentPosition?.fen);
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
          game.validMoves = await getValidMoves(move.position?.fen);
        }
      }
    },
  },
});
