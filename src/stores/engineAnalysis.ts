import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { engineAnalysisEventService } from "../services/AnalysisService";
import api from "../shared/api";
import type {
  AnalysisUpdate,
  BestMove,
  BestMovePayload,
  EngineSettings,
} from "../shared/types";

interface EngineState {
  name: string;
  isAnalyzing: boolean;
  analysisUpdates: (AnalysisUpdate & { timestamp: number })[];
  bestMoves: (BestMove & { timestamp: number })[];
  engineSettings: EngineSettings;
}

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
      if (engine) {
        const analysisUpdates = engine.analysisUpdates;
        if (analysisUpdates.length > 0) {
          return analysisUpdates[analysisUpdates.length - 1];
        }
      }
      return undefined;
    },
    getLatestBestMove: (state) => (engineName: string) => {
      const engine = state.engines.get(engineName);
      if (engine) {
        const bestMoves = engine.bestMoves;
        if (bestMoves.length > 0) {
          return bestMoves[bestMoves.length - 1];
        }
      }
      return undefined;
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
      this.engines.set(engineName, {
        name: engineName,
        isAnalyzing: false,
        analysisUpdates: [],
        bestMoves: [],
        engineSettings: {},
      });
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
        console.log("Analysis update", update);
        engine.analysisUpdates.push({
          ...update,
          timestamp: Date.now(),
        });
      }
    },
    addBestMove(engineName: string, bestMove: BestMovePayload) {
      const engine = this.engines.get(engineName);
      if (engine) {
        const bestMoveMove = bestMove[0];
        const bestMovePonder = bestMove[1];
        engine.bestMoves.push({
          move: bestMoveMove,
          ponder: bestMovePonder,
          timestamp: Date.now(),
        });
      }
    },
    addAnalysisListener(
      engineName: string,
      listener: (result: AnalysisUpdate) => void
    ) {
      if (!this.analysisListeners.has(engineName)) {
        this.analysisListeners.set(engineName, new Set());
      }
      this.analysisListeners.get(engineName)?.add(listener);
    },
    removeAnalysisListener(
      engineName: string,
      listener: (result: AnalysisUpdate) => void
    ) {
      this.analysisListeners.get(engineName)?.delete(listener);
    },
    setEngineSettings(engineName: string, settings: EngineSettings) {
      const engine = this.engines.get(engineName);
      if (engine) {
        // Apply all default option values
        for (const [key, option] of Object.entries(settings)) {
          settings[key] = { ...option, value: option.default };
        }
        engine.engineSettings = settings;
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
      await api.engines.POST.loadEngine(name, path);
    },
    async unloadEngine(name: string) {
      await api.engines.POST.unloadEngine(name);
      this.removeEngine(name);
    },
    async setEngineOption(engineName: string, option: string, value: string) {
      await invoke("set_engine_option", { engineName, option, value });
    },
    async updateEngineSettings(
      engineName: string,
      settings: Partial<EngineSettings>
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
      timeMs?: number
    ) {
      if (!this.engines.has(engineName))
        throw new Error(`Engine ${engineName} not loaded`);
      await api.engines.POST.analyzePosition(
        engineName,
        fen,
        depth ?? 20,
        timeMs ?? 10000
      );
      this.setEngineAnalyzing(engineName, true);
    },
    async stopAnalysis(engineName: string) {
      if (!this.engines.has(engineName))
        throw new Error(`Engine ${engineName} not loaded`);
      await api.engines.POST.stopAnalysis(engineName);
      this.setEngineAnalyzing(engineName, false);
    },
    async analyzeGame(engineName: string, gameId: number) {
      await invoke("analyze_game", { engineName, gameId });
    },
  },
});
