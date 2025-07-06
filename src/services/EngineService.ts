import { debug, info } from "@tauri-apps/plugin-log";
import { API } from "../shared/api";
import type {
	AnalysisUpdate,
	BestMove,
	BestMovePayload,
	EngineSettings,
	Score,
} from "../shared/types";
import type { OperationResult } from "../shared/types";
import { ErrorCategory, withErrorHandling } from "./ErrorService";

/**
 * Engine state information
 */
export interface EngineState {
	name: string;
	isAnalyzing: boolean;
	analysisUpdates: (AnalysisUpdate & { timestamp: number })[];
	bestMoves: (BestMove & { timestamp: number })[];
	engineSettings: EngineSettings;
}

/**
 * Load a chess engine
 */
export async function loadEngine(
	name: string,
	path: string,
): Promise<OperationResult> {
	return await withErrorHandling(
		async () => API.analysis.loadEngine(name, path),
		ErrorCategory.CHESS_ENGINE,
		"ENGINE_LOAD_ERROR",
		`Failed to load engine ${name} from ${path}`,
		{
			metadata: { engineName: name, enginePath: path },
		},
	);
}

/**
 * Unload a chess engine
 */
export async function unloadEngine(name: string): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.analysis.unloadEngine(name),
		ErrorCategory.CHESS_ENGINE,
		"ENGINE_UNLOAD_ERROR",
		`Failed to unload engine ${name}`,
		{
			metadata: { engineName: name },
		},
	);
}

/**
 * Set an engine option
 */
export async function setEngineOption(
	engineName: string,
	option: string,
	value: string,
): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.analysis.setEngineOption(engineName, option, value),
		ErrorCategory.CHESS_ENGINE,
		"ENGINE_OPTION_ERROR",
		`Failed to set engine option ${option}=${value} for ${engineName}`,
		{
			metadata: { engineName, option, value },
		},
	);
}

/**
 * Start position analysis
 */
export async function analyzePosition(
	engineName: string,
	fen: string,
	depth = 20,
	timeMs = 10000,
): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.analysis.analyze(engineName, fen, { depth, timeMs }),
		ErrorCategory.CHESS_ENGINE,
		"ENGINE_ANALYSIS_ERROR",
		`Failed to analyze position for ${engineName}`,
		{
			metadata: { engineName, fen, depth, timeMs },
		},
	);
}

/**
 * Stop position analysis
 */
export async function stopAnalysis(
	engineName: string,
): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.analysis.stopAnalysis(engineName),
		ErrorCategory.CHESS_ENGINE,
		"ENGINE_ANALYSIS_ERROR",
		`Failed to stop analysis for ${engineName}`,
		{
			metadata: { engineName },
		},
	);
}

/**
 * Start game analysis
 */
export async function analyzeGame(boardId: number): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.analysis.analyzeGame(boardId),
		ErrorCategory.CHESS_ENGINE,
		"ENGINE_ANALYSIS_ERROR",
		`Failed to analyze game ${boardId}`,
		{
			metadata: { boardId },
		},
	);
}

/**
 * Get the latest analysis update for an engine
 */
export function getLatestAnalysisUpdate(
	engineState: EngineState,
): (AnalysisUpdate & { timestamp: number }) | null {
	const analysisUpdates = engineState.analysisUpdates;
	if (analysisUpdates.length > 0) {
		return analysisUpdates[analysisUpdates.length - 1];
	}
	return null;
}

/**
 * Get the latest best move for an engine
 */
export function getLatestBestMove(
	engineState: EngineState,
): (BestMove & { timestamp: number }) | null {
	const bestMoves = engineState.bestMoves;
	if (bestMoves.length > 0) {
		// Check if the engine's most recent analysis update came after
		// the most recent best move. If it did, then the best move
		// may not be valid anymore (e.g. the position has changed)
		// TODO: consider using a more robust way to determine if the best move is still valid
		// for the current state. For now, we just wait for a new best move.
		const latestAnalysisUpdate = getLatestAnalysisUpdate(engineState);
		if (latestAnalysisUpdate) {
			const latestBestMove = bestMoves[bestMoves.length - 1];
			if (latestAnalysisUpdate.timestamp > latestBestMove.timestamp) {
				return null;
			}
			return latestBestMove;
		}
	}
	return null;
}

/**
 * Calculate board evaluation from engine analysis
 */
export function calculateBoardEvaluation(
	engines: Map<string, EngineState>,
): Score {
	// FIXME: Provide a way to set which engine to use for evaluation
	// for now just use the first engine
	const engine = engines.values().next().value;
	if (engine) {
		const analysisUpdates = engine.analysisUpdates;
		if (analysisUpdates.length > 0) {
			const lastUpdate = analysisUpdates[analysisUpdates.length - 1];

			const evaluation = {
				value: lastUpdate.score?.value,
				type: lastUpdate.score?.type ?? "centipawns",
			};
			if (evaluation.value !== undefined) {
				info(`Evaluation: ${evaluation}`);
				return evaluation as Score;
			}
		}
	}
	return { value: 0, type: "centipawns" };
}

/**
 * Apply default values to engine settings
 */
export function applyDefaultEngineSettings(
	settings: EngineSettings,
): EngineSettings {
	const updatedSettings = { ...settings };

	// Apply all default option values
	for (const [key, option] of Object.entries(updatedSettings)) {
		updatedSettings[key] = { ...option, value: option.default };
	}

	return updatedSettings;
}

/**
 * Create a new engine state
 */
export function createEngineState(engineName: string): EngineState {
	return {
		name: engineName,
		isAnalyzing: false,
		analysisUpdates: [],
		bestMoves: [],
		engineSettings: {},
	};
}

/**
 * Add analysis update to engine state
 */
export function addAnalysisUpdate(
	engineState: EngineState,
	update: AnalysisUpdate,
): void {
	debug(
		`Adding analysis update for engine ${engineState.name}: ${JSON.stringify(update)}`,
	);
	engineState.analysisUpdates.push({
		...update,
		timestamp: Date.now(),
	});
}

/**
 * Add best move to engine state
 */
export function addBestMove(
	engineState: EngineState,
	bestMove: BestMovePayload,
): void {
	const bestMoveMove = bestMove[0];
	const bestMovePonder = bestMove[1];
	engineState.bestMoves.push({
		move: bestMoveMove,
		ponder: bestMovePonder,
		timestamp: Date.now(),
	});
}

/**
 * Update engine settings
 */
export function updateEngineSettings(
	engineState: EngineState,
	settings: Partial<EngineSettings>,
): void {
	const currentSettings = engineState.engineSettings;
	engineState.engineSettings = applyDefaultEngineSettings({
		...currentSettings,
		...settings,
	} as EngineSettings);
}
