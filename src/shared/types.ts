import typia from "typia";
import type {
	ChessGame,
	ChessMove,
	ChessMoveTree,
	ChessTreeNode,
	LegalMove,
} from "./bindings";

////////////////////////////////////////////////////////////
// Application UI Interfaces
////////////////////////////////////////////////////////////

// Moved to `themes.ts` as it was only theme related,
// generally types/constants could use some better organization
// but that's a 'todo' for another day

export interface ActiveGameState {
	id: number;
	game: ChessGame;

	// UI state
	hideEvaluationBar: boolean;
	hideBestMove: boolean;
	hideThreats: boolean;

	// Loading states
	isLoading: boolean;
	error: string | null;
}

export interface NodeId {
	idx: number;
	version: number;
}

export interface MoveData {
	nodeId: NodeId;
	node: ChessTreeNode;
	move?: ChessMove;
	san: string;
	plyNumber: number;
	moveNumber: number;
	showNumber: boolean;
	isWhite: boolean;
	isMainLine: boolean;
	isVariation: boolean;
	depth: number;
	parentMoveNumber: number | null;
}

export interface MoveGroup {
	mainMoves: MoveData[];
	variations: MoveData[][];
}

export interface TableMoveRow {
	type: "move";
	number: number;
	white?: MoveData;
	black?: MoveData;
}

export type VariationMove = MoveData | TableVariationRow;

export interface TableVariationRow {
	type: "variation";
	moves: VariationMove[];
	depth: number;
	collapsible?: boolean;
}

export type TableRow = TableMoveRow | TableVariationRow;

export type ViewMode = "compact" | "tabular";

export interface MoveDisplayProps {
	moveTree: ChessMoveTree;
}

export interface MoveDisplayEmits {
	"select-move": [move_id: number];
	"navigate-start": [];
	"navigate-end": [];
	"navigate-previous": [];
	"navigate-next": [variation_idx: number];
}

export interface AlertToast {
	key: string;
	type: "success" | "error" | "info" | "warning";
	message: string;
	title?: string;
	timeout?: number;
}

/**
 * Metadata for an open chess board
 */
export interface BoardMetadata {
	name: string;
	hasUnsavedChanges: boolean;
}

/**
 * Sorting options for the game library
 */
export type SortOption =
	| "date"
	| "event"
	| "white"
	| "black"
	| "result"
	| "opening";

export interface SortConfig {
	field: SortOption;
	order: "asc" | "desc";
}

/**
 * Filter options for the game library
 */
export type FilterOption = "all" | "favorites" | "tags";

////////////////////////////////////////////////////////////
// Application Interfaces
// Api interfaces are mostly sub-sets of backend models (bindings)
////////////////////////////////////////////////////////////

/**
 * Fields of `ChessGame` model that are displayed in the explorer/library view
 */
export const explorerGameFields: (keyof ChessGame)[] = [
	"id",
	"white_player",
	"black_player",
	"tournament",
	"opening",
	"result",
	"round",
	"date",
	"tags",
] as const;

/**
 * A game displayed in the explorer/library view
 */
export type ExplorerGame = Pick<ChessGame, (typeof explorerGameFields)[number]>;

/**
 * Parse a JSON string into an `ExplorerGame` object
 */
export const parseExplorerGame = typia.json.createValidateParse<ExplorerGame>();

/**
 * Parse a JSON string into an array of `ExplorerGame` objects
 */
export const parseExplorerGames =
	typia.json.createValidateParse<ExplorerGame[]>();

/**
 * Parse a JSON string into an array of `LegalMove` objects
 */
export const parseLegalMoves = typia.json.createValidateParse<LegalMove[]>();

/**
 * Events emitted by the engine
 */
export interface EngineEvents {
	analysisUpdate: AnalysisUpdate;
	bestMove: BestMove;
}

export interface BestMove {
	move: string;
	ponder?: string;
	timestamp: number;
}

/**
 * Best move of the engine's analysis
 */
export type BestMovePayload = [move: string, ponder?: string];

/**
 * Score of the engine's analysis
 */
export interface Score {
	value: number;
	type?: "centipawns" | "mate";
}

/**
 * Update of the engine's analysis
 */
export interface AnalysisUpdate {
	depth?: number;
	seldepth?: number;
	time?: number;
	nodes?: number;
	pv?: string[];
	multipv?: number;
	score?: Score;
	hashfull?: number;
	nps?: number;
	tbhits?: number;
}

/**
 * Analysis update event payload
 */
export type AnalysisUpdatePayload = [
	engineName: string,
	update:
		| {
				AnalysisUpdate: AnalysisUpdate;
		  }
		| {
				BestMove: BestMovePayload;
		  },
];

/**
 * Parse a JSON string into an `AnalysisUpdatePayload` object
 */
export const parseAnalysisUpdatePayload =
	typia.json.createValidateParse<AnalysisUpdatePayload>();

/**
 * Engine option
 */
export interface EngineOption {
	type: "check" | "spin" | "combo" | "button" | "string";
	value?: string;
	default?: string;
	min?: number;
	max?: number;
	var?: string[];
}

/**
 * Engine options
 */
export interface EngineSettings {
	[name: string]: EngineOption;
}

/**
 * Engine settings event payload
 */
export type EngineSettingsPayload = [
	engineName: string,
	EngineSettings: EngineSettings,
];

/**
 * Parse a JSON string into an `EngineSettingsPayload` object
 */
export const parseEngineSettingsPayload =
	typia.json.createValidateParse<EngineSettingsPayload>();

/**
 * Parse a chess game
 */
export const parseChessGame = typia.json.createValidateParse<ChessGame>();

////////////////////////////////////////////////////////////
// Error Types
////////////////////////////////////////////////////////////

/**
 * A generic result of an operation
 */
export interface OperationResult<T = void> {
	success: boolean;
	data?: T;
	error?: string;
}
