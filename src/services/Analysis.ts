import { invoke } from "@tauri-apps/api/core";
import { type UnlistenFn, listen } from "@tauri-apps/api/event";
import type { ChessGame, ChessMove } from "../shared/bindings";

export interface AnalysisResult {
  fen: string;
  depth: number;
  score: number;
  bestMove: string;
  pv: string[];
  nodes: number;
  time: number;
}

export interface EngineSettings {
  // Core options, just typed specifically
  // for easier type checking (might want to remove these in the future)
  depth: EngineOption;
  multiPV: EngineOption;
  threads: EngineOption;
  hashSize: EngineOption;

  // Any additional options
  [name: string]: EngineOption | undefined;
}

export interface EngineOption {
  option_type: "Check" | "Spin" | "Combo" | "Button" | "String";
  value: string | null;
  default: string | null;
  min: number | null;
  max: number | null;
  var: string[] | null;
}

class EngineAnalysisService {
  private engines: Map<string, boolean> = new Map(); // engine name -> isAnalyzing
  private engineFens: Map<string, string | null> = new Map(); // engine name -> currently set FEN
  private analysisListeners: Map<
    string,
    Set<(result: AnalysisResult) => void>
  > = new Map();
  private engineSettings: Map<string, EngineSettings> = new Map();
  private gameAnalysisInProgress = false;
  private registeredEventUnlisteners: UnlistenFn[] = [];

  constructor() {
    this.setupListeners();

    this.engineSettings.set("stockfish", {
      depth: {
        option_type: "Spin",
        default: "18",
        min: 1,
        max: 100,
        var: null,
        value: null,
      },
      multiPV: {
        option_type: "Spin",
        default: "3",
        min: 1,
        max: 100,
        var: null,
        value: null,
      },
      threads: {
        option_type: "Spin",
        default: "4",
        min: 1,
        max: 100,
        var: null,
        value: null,
      },
      hashSize: {
        option_type: "Spin",
        default: "128",
        min: 1,
        max: 100,
        var: null,
        value: null,
      },
    });
  }

  destroy() {
    for (const unlisten of this.registeredEventUnlisteners) {
      unlisten();
    }
  }

  private async setupListeners() {
    // Listen for engine output
    this.registeredEventUnlisteners.push(
      await listen<string>("engine-output", (event) => {
        console.log("Engine output", event.payload);
      })
    );

    // Listen for engine initialization/options
    this.registeredEventUnlisteners.push(
      await listen<string>("engine-options", (event) => {
        console.log("Engine options", event.payload);
        // TODO: Update engine settings
      })
    );

    // Listen for game analysis progress
    this.registeredEventUnlisteners.push(
      await listen<string>("game-analysis-progress", (event) => {
        console.log("Game analysis progress", event.payload);
      })
    );

    // Listen for game analysis complete
    this.registeredEventUnlisteners.push(
      await listen<string>("game-analysis-complete", (event) => {
        console.log("Game analysis complete", event.payload);
      })
    );
  }

  async loadEngine(name: string, path: string): Promise<void> {
    try {
      await invoke("load_engine", { name, path });
    } catch (error) {
      // // if the error is that it's already loaded, then we can
      // // continue as normal. This will be seen in the `EngineError` property
      // if (
      //   error instanceof Error &&
      //   "EngineError" in error &&
      //   typeof error.EngineError === "string" &&
      //   error.EngineError.includes("already loaded")
      // ) {
      //   console.log("Engine already loaded");
      // } else {
      //   throw error;
      // }
      // TODO: Handle error correctly
    }
    this.engines.set(name, false);
    this.engineFens.set(name, null);
  }

  async setEngineOption(
    engineName: string,
    option: string,
    value: string
  ): Promise<void> {
    await invoke("set_engine_option", { engineName, option, value });
  }

  async updateEngineSettings(
    engineName: string,
    settings: Partial<EngineSettings>
  ): Promise<void> {
    const currentSettings = this.engineSettings.get(engineName);
    if (!currentSettings) return;

    this.engineSettings.set(engineName, { ...currentSettings, ...settings });

    // Apply settings
    const newSettings = this.engineSettings.get(engineName);
    if (newSettings && settings.multiPV !== undefined) {
      await this.setEngineOption(
        engineName,
        "MultiPV",
        newSettings.multiPV.toString()
      );
    }
    if (newSettings && settings.threads !== undefined) {
      await this.setEngineOption(
        engineName,
        "Threads",
        newSettings.threads.toString()
      );
    }
    if (newSettings && settings.hashSize !== undefined) {
      await this.setEngineOption(
        engineName,
        "Hash",
        newSettings.hashSize.toString()
      );
    }
  }

  async addAnalysisListener(
    engineName: string,
    listener: (result: AnalysisResult) => void
  ): Promise<void> {
    if (!this.analysisListeners.has(engineName)) {
      this.analysisListeners.set(engineName, new Set());
    }
    this.analysisListeners.get(engineName)?.add(listener);
  }

  async removeAnalysisListener(
    engineName: string,
    listener: (result: AnalysisResult) => void
  ): Promise<void> {
    this.analysisListeners.get(engineName)?.delete(listener);
  }

  async analyzePosition(
    engineName: string,
    fen: string,
    onAnalysisResult?: (result: AnalysisResult) => void
  ): Promise<void> {
    if (!this.engines.has(engineName)) {
      throw new Error(`Engine ${engineName} not loaded`);
    }

    // Stop any previous analysis
    if (this.engines.get(engineName)) {
      await this.stopAnalysis(engineName);
    }

    // Register listener
    if (onAnalysisResult) {
      if (!this.analysisListeners.has(engineName)) {
        this.analysisListeners.set(engineName, new Set());
      }
      this.analysisListeners.get(engineName)?.add(onAnalysisResult);
    }

    // Set position
    await invoke("set_position", { fen });
    this.engineFens.set(engineName, fen);

    // Start analysis
    const settings = this.engineSettings.get(engineName);
    if (settings) {
      await invoke("analyze_position", { fen, depth: 10 });
    }

    this.engines.set(engineName, true);
  }

  async stopAnalysis(engineName: string): Promise<void> {
    if (!this.engines.has(engineName)) {
      throw new Error(`Engine ${engineName} not loaded`);
    }

    if (this.engines.get(engineName)) {
      await invoke("stop_analysis", {});
      this.engines.set(engineName, false);
    }
  }

  async analyzeGame(
    engineName: string,
    gameId: number
  ): Promise<Map<number, AnalysisResult>> {
    await invoke("analyze_game", { engineName, gameId });

    return new Map();
  }

  isEngineAnalyzing(engineName: string): boolean {
    return this.engines.get(engineName) || false;
  }

  getEngineSettings(engineName: string): EngineSettings | undefined {
    return this.engineSettings.get(engineName);
  }

  isGameAnalysisInProgress(): boolean {
    return this.gameAnalysisInProgress;
  }
}

export const engineAnalysisService = new EngineAnalysisService();
export default engineAnalysisService;
