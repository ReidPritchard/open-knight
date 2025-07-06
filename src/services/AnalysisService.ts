import { type UnlistenFn, listen } from "@tauri-apps/api/event";
import {
	parseAnalysisUpdatePayload,
	parseEngineSettingsPayload,
} from "../shared/types";
import type { useEngineAnalysisStore } from "../stores/engineAnalysis";
import { warn } from "@tauri-apps/plugin-log";

class EngineAnalysisEventService {
	initialized = false;
	private registeredEventUnlisteners: UnlistenFn[] = [];

	async setupListeners(store: ReturnType<typeof useEngineAnalysisStore>) {
		if (this.initialized) return;
		this.initialized = true;
		// Listen for engine output
		this.registeredEventUnlisteners.push(
			await listen<string>("engine-output", (event) => {
				try {
					const parseResult = parseAnalysisUpdatePayload(event.payload);
					if (parseResult.success) {
						const [engineName, update] = parseResult.data;
						if ("AnalysisUpdate" in update) {
							store.addAnalysisUpdate(engineName, update.AnalysisUpdate);
							const listeners = store.analysisListeners.get(engineName);
							if (listeners) {
								for (const cb of listeners) {
									cb(update.AnalysisUpdate);
								}
							}
						} else if ("BestMove" in update) {
							store.addBestMove(engineName, update.BestMove);
						}
					}
				} catch (error) {
					warn(`Error parsing engine output: ${error}`);
				}
			}),
		);
		// Listen for engine options
		this.registeredEventUnlisteners.push(
			await listen<string>("engine-options", (event) => {
				const parseResult = parseEngineSettingsPayload(event.payload);
				if (parseResult.success) {
					const [engineName, options] = parseResult.data;
					store.setEngineSettings(engineName, options);
				}
			}),
		);
		// Listen for game analysis progress/complete (optional: implement as needed)
	}

	destroy() {
		for (const unlisten of this.registeredEventUnlisteners) {
			unlisten();
		}
		this.registeredEventUnlisteners = [];
		this.initialized = false;
	}
}

export const engineAnalysisEventService = new EngineAnalysisEventService();
