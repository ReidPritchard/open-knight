import { invoke } from "@tauri-apps/api/core";
import api from "../shared/api";
import type {
	AnalysisUpdate,
	BestMove,
	BestMovePayload,
	EngineSettings,
	Score,
} from "../shared/types";
import { ErrorFactory, ErrorHandler } from "./ErrorService";

/**
 * Result of an engine operation
 */
export interface EngineOperationResult<T = void> {
	success: boolean;
	data?: T;
	error?: string;
}

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
): Promise<EngineOperationResult> {
	try {
		await api.engines.POST.loadEngine(name, path);
		console.log("Engine loaded successfully:", name);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to load engine";
		ErrorHandler.handle(
			ErrorFactory.chessEngine(
				"ENGINE_LOAD_ERROR",
				`Failed to load engine ${name} from ${path}: ${errorMessage}`,
				{
					metadata: { engineName: name, enginePath: path },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Unload a chess engine
 */
export async function unloadEngine(
	name: string,
): Promise<EngineOperationResult> {
	try {
		await api.engines.POST.unloadEngine(name);
		console.log("Engine unloaded successfully:", name);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to unload engine";
		ErrorHandler.handle(
			ErrorFactory.chessEngine(
				"ENGINE_UNLOAD_ERROR",
				`Failed to unload engine ${name}: ${errorMessage}`,
				{
					metadata: { engineName: name },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Set an engine option
 */
export async function setEngineOption(
	engineName: string,
	option: string,
	value: string,
): Promise<EngineOperationResult> {
	try {
		await invoke("set_engine_option", { engineName, option, value });
		console.log("Engine option set:", engineName, option, value);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to set engine option";
		ErrorHandler.handle(
			ErrorFactory.chessEngine(
				"ENGINE_PROTOCOL_ERROR",
				`Failed to set engine option ${option}=${value} for ${engineName}: ${errorMessage}`,
				{
					metadata: { engineName, option, value },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Start position analysis
 */
export async function analyzePosition(
	engineName: string,
	fen: string,
	depth = 20,
	timeMs = 10000,
): Promise<EngineOperationResult> {
	try {
		await api.engines.POST.analyzePosition(engineName, fen, depth, timeMs);
		console.log("Position analysis started:", engineName, fen);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to analyze position";
		ErrorHandler.handle(
			ErrorFactory.chessEngine(
				"ENGINE_PROTOCOL_ERROR",
				`Failed to analyze position for ${engineName}: ${errorMessage}`,
				{
					metadata: { engineName, fen, depth, timeMs },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Stop position analysis
 */
export async function stopAnalysis(
	engineName: string,
): Promise<EngineOperationResult> {
	try {
		await api.engines.POST.stopAnalysis(engineName);
		console.log("Analysis stopped:", engineName);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to stop analysis";
		ErrorHandler.handle(
			ErrorFactory.chessEngine(
				"ENGINE_PROTOCOL_ERROR",
				`Failed to stop analysis for ${engineName}: ${errorMessage}`,
				{
					metadata: { engineName },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Start game analysis
 */
export async function analyzeGame(
	engineName: string,
	gameId: number,
): Promise<EngineOperationResult> {
	try {
		await invoke("analyze_game", { engineName, gameId });
		console.log("Game analysis started:", engineName, gameId);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to analyze game";
		ErrorHandler.handle(
			ErrorFactory.chessEngine(
				"ENGINE_PROTOCOL_ERROR",
				`Failed to analyze game ${gameId} with ${engineName}: ${errorMessage}`,
				{
					metadata: { engineName, gameId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
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
				console.log("Evaluation", evaluation);
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
	console.log("Analysis update", update);
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
