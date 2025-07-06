import { defineStore } from "pinia";
import { engineAnalysisEventService } from "../services/AnalysisService";
import * as EngineService from "../services/EngineService";
import type { EngineState } from "../services/EngineService";
import type {
	AnalysisUpdate,
	BestMovePayload,
	EngineSettings,
	Score,
} from "../shared/types";

export const useEngineAnalysisStore = defineStore("engineAnalysis", {
	state: () => ({
		engines: new Map<string, EngineState>(),
		analysisListeners: new Map<string, Set<(result: AnalysisUpdate) => void>>(),
		gameAnalysisInProgress: false,
		analysisService: engineAnalysisEventService,
	}),
	getters: {
		getEngineSettings: (state) => (engineName: string) =>
			state.engines.get(engineName)?.engineSettings ?? {},
		getLatestAnalysisUpdate: (state) => (engineName: string) => {
			const engine = state.engines.get(engineName);
			if (!engine) return undefined;
			return EngineService.getLatestAnalysisUpdate(engine);
		},
		getLatestBestMove: (state) => (engineName: string) => {
			const engine = state.engines.get(engineName);
			if (!engine) return undefined;
			return EngineService.getLatestBestMove(engine);
		},
		isAnalyzing: (state) => (engineName: string) => {
			return state.engines.get(engineName)?.isAnalyzing ?? false;
		},
		boardEvaluation: (state): Score => {
			return EngineService.calculateBoardEvaluation(state.engines);
		},
	},
	actions: {
		initAnalysisService() {
			if (this.analysisService.initialized) return;
			this.analysisService.setupListeners(this);
		},
		initEngine(engineName: string) {
			if (!this.analysisService.initialized) {
				this.initAnalysisService();
			}

			if (this.engines.has(engineName)) {
				console.warn("Engine already initialized", engineName);
				return;
			}

			console.log("Initializing engine", engineName);
			this.engines.set(engineName, EngineService.createEngineState(engineName));
		},
		removeEngine(engineName: string) {
			this.engines.delete(engineName);
			this.analysisListeners.delete(engineName);
		},
		setEngineAnalyzing(engineName: string, isAnalyzing: boolean) {
			const engine = this.engines.get(engineName);
			if (engine) {
				engine.isAnalyzing = isAnalyzing;
			}
		},
		addAnalysisUpdate(engineName: string, update: AnalysisUpdate) {
			const engine = this.engines.get(engineName);
			if (engine) {
				EngineService.addAnalysisUpdate(engine, update);
				// Setting an analysis update indicates the engine is still analyzing the current position
				this.setEngineAnalyzing(engineName, true);
				// It also means any "best move" is no longer valid
			}
		},
		addBestMove(engineName: string, bestMove: BestMovePayload) {
			const engine = this.engines.get(engineName);
			if (engine) {
				EngineService.addBestMove(engine, bestMove);
				// Setting a best move indicates the engine is done analyzing the current position
				this.setEngineAnalyzing(engineName, false);
			}
		},
		addAnalysisListener(
			engineName: string,
			listener: (result: AnalysisUpdate) => void,
		) {
			if (!this.analysisListeners.has(engineName)) {
				this.analysisListeners.set(engineName, new Set());
			}
			this.analysisListeners.get(engineName)?.add(listener);
		},
		removeAnalysisListener(
			engineName: string,
			listener: (result: AnalysisUpdate) => void,
		) {
			this.analysisListeners.get(engineName)?.delete(listener);
		},
		setEngineSettings(engineName: string, settings: EngineSettings) {
			const engine = this.engines.get(engineName);
			if (engine) {
				EngineService.updateEngineSettings(engine, settings);
			} else {
				console.warn("Engine not found", engineName);
				this.initEngine(engineName);
				this.setEngineSettings(engineName, settings);
			}
		},
		setGameAnalysisInProgress(inProgress: boolean) {
			this.gameAnalysisInProgress = inProgress;
		},
		reset() {
			this.engines.clear();
			this.analysisListeners.clear();
			this.gameAnalysisInProgress = false;
		},
		async loadEngine(name: string, path: string) {
			if (this.engines.has(name)) return;
			this.initEngine(name);
			const result = await EngineService.loadEngine(name, path);
			if (!result.success) {
				this.removeEngine(name);
			}
		},
		async unloadEngine(name: string) {
			const result = await EngineService.unloadEngine(name);
			if (result.success) {
				this.removeEngine(name);
			}
		},
		async setEngineOption(engineName: string, option: string, value: string) {
			await EngineService.setEngineOption(engineName, option, value);
		},
		async updateEngineSettings(
			engineName: string,
			settings: Partial<EngineSettings>,
		) {
			const currentSettings = this.engines.get(engineName)?.engineSettings;
			if (!currentSettings) return;
			this.setEngineSettings(engineName, {
				...currentSettings,
				...settings,
			} as EngineSettings);
		},
		async analyzePosition(
			engineName: string,
			fen: string,
			depth?: number,
			timeMs?: number,
		) {
			if (!this.engines.has(engineName))
				throw new Error(`Engine ${engineName} not loaded`);
			const result = await EngineService.analyzePosition(
				engineName,
				fen,
				depth ?? 20,
				timeMs ?? 10000,
			);
			if (result.success) {
				this.setEngineAnalyzing(engineName, true);
			}
		},
		async stopAnalysis(engineName: string) {
			if (!this.engines.has(engineName))
				throw new Error(`Engine ${engineName} not loaded`);
			const result = await EngineService.stopAnalysis(engineName);
			if (result.success) {
				this.setEngineAnalyzing(engineName, false);
			}
		},
		async analyzeGame(boardId: number) {
			await EngineService.analyzeGame(boardId);
		},
	},
});
