import { defineStore } from "pinia";
import api from "../shared/api";
import type {
  ChessGame,
  ChessPosition,
  ChessTreeNode,
  LegalMove,
} from "../shared/bindings";

interface ActiveGameState {
  id: number;
  game: ChessGame;

  currentMoveIndex: number;
  currentMove: ChessTreeNode | null;
  currentPosition: ChessPosition | null;
  currentTurn: "white" | "black" | null;
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

const getTurnFromFen = (fen: string): "white" | "black" | null => {
  const turn = fen.split(" ")[1];
  if (!turn) return null;
  return turn === "w" ? "white" : "black";
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
      // TODO: Check if game is already open in another board
      // If so, maybe navigate to that board?

      if (this.activeGameMap.has(boardId)) {
        // A game is already open on this board
        // Clear the board to avoid old state from staying around
        this.activeGameMap.delete(boardId);
      }

      // Open game
      const game = await api.games.GET.game(gameId);
      const initialPosition: ChessPosition = {
        id: 0,
        fen: game.fen ?? "",
        evaluations: [],
        variant: "Standard", // TODO: Handle variations
      };
      const validMoves = await getValidMoves(initialPosition?.fen);

      console.log("Game:", game);

      const newGameState: ActiveGameState = {
        id: gameId,
        game: game,

        currentMoveIndex: 0,
        currentMove: null,
        currentPosition: initialPosition,
        currentTurn: "white",

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
      const currentMoveId = game.game.move_tree.current_node_id?.idx ?? 0;
      const currentMove = game.game.move_tree.nodes[currentMoveId];

      const nextMoveId = currentMove.value?.children_ids[0]?.idx;
      if (!nextMoveId) return;

      const nextMove = game.game.move_tree.nodes[nextMoveId];
      if (!nextMove || !nextMove.value) return;

      game.game.move_tree.current_node_id = {
        idx: nextMoveId,
        version: nextMove.version,
      };

      // Try to get the turn from the FEN (that way we are positive it's correct)
      let nextTurn = getTurnFromFen(nextMove.value.position.fen);
      if (!nextTurn && game.currentTurn) {
        nextTurn = game.currentTurn === "white" ? "black" : "white";
      } else if (!nextTurn) {
        nextTurn = "white";
      }

      game.currentMove = nextMove.value;
      game.currentMoveIndex = nextMoveId;
      game.currentPosition = nextMove.value.position;
      game.currentTurn = nextTurn as "white" | "black";
      game.validMoves = await getValidMoves(nextMove.value?.position?.fen);

      console.log("Current move:", game.currentMove);
    },

    async previousMove(boardId: number) {
      const game = this.activeGameMap.get(boardId);
      if (game) {
        // Get the current move (to use it's parent id)
        const currentMoveId = game.game.move_tree.current_node_id?.idx ?? 0;
        const currentMove = game.game.move_tree.nodes[currentMoveId];

        // Get the previous move
        const previousMoveId = currentMove.value?.parent_id?.idx;
        if (!previousMoveId) return;

        const previousMove = game.game.move_tree.nodes[previousMoveId];
        if (!previousMove || !previousMove.value) return;

        game.game.move_tree.current_node_id = {
          idx: previousMoveId,
          version: previousMove.version,
        };

        // Try to get the turn from the FEN (that way we are positive it's correct)
        let previousTurn = getTurnFromFen(previousMove.value.position.fen);
        if (!previousTurn && game.currentTurn) {
          previousTurn = game.currentTurn === "white" ? "black" : "white";
        } else if (!previousTurn) {
          previousTurn = "white";
        }

        game.currentMove = previousMove.value;
        game.currentMoveIndex = previousMoveId;
        game.currentPosition = previousMove.value.position;
        game.currentTurn = previousTurn as "white" | "black";
        game.validMoves = await getValidMoves(
          previousMove.value?.position?.fen
        );

        console.log("Previous move:", game.currentMove);
      }
    },

    async jumpToMove(boardId: number, moveId: number) {
      const game = this.activeGameMap.get(boardId);
      if (game) {
        let moveIndex: number | null = null;
        // Find the move in the game
        const found_move = game.game.move_tree.nodes.find(
          (search_move, search_move_index) => {
            if (search_move.value?.game_move?.id === moveId) {
              moveIndex = search_move_index;
              return true;
            }
            return false;
          }
        );
        if (!found_move || !found_move.value || !moveIndex) return;

        // Try to get the turn from the FEN (that way we are positive it's correct)
        let jumpTurn = getTurnFromFen(found_move.value.position.fen);
        if (!jumpTurn) {
          // Try to use the turn's ply to determine the turn
          jumpTurn =
            found_move.value.position.ply % 2 === 0 ? "white" : "black";
        }

        game.game.move_tree.current_node_id = {
          idx: moveIndex,
          version: found_move.version,
        };
        game.currentMove = found_move.value;
        game.currentPosition = found_move.value?.position;
        game.currentTurn = jumpTurn as "white" | "black";
        game.validMoves = await getValidMoves(found_move.value?.position?.fen);
      }
    },
  },
});
