import { createPinia } from "pinia";

import "./style.css";

import { createApp } from "vue";
import App from "./App.vue";
import { ErrorHandler, ErrorSeverity } from "./services/ErrorService";
import { useGlobalStore } from "./stores";
import { useSettingsStore } from "./stores/settings";
import { useAppProviders } from "./composables/useProviders";
import {
	warn,
	debug,
	trace,
	info,
	error,
	attachConsole,
} from "@tauri-apps/plugin-log";

/**
 * Main function of the application
 * Only added due to build errors caused by top-level await support
 * though likely these could be avoided by tweaking the build config.
 * TODO: Review build configuration and remove this function
 */
async function main() {
	// Setup logging
	// TODO: Remove this and use the log functions directly
	function forwardConsole(
		fnName: "log" | "debug" | "info" | "warn" | "error",
		logger: (message: string) => Promise<void>,
	) {
		// eslint-disable-next-line no-console
		const original = console[fnName];
		// eslint-disable-next-line no-console
		console[fnName] = (...data: unknown[]) => {
			original(...data);
			// convert data series into a string
			const message = data
				.map((item) => {
					if (typeof item === "object" && item !== null) {
						return JSON.stringify(item);
					}
					return item;
				})
				.join(" ");
			logger(message);
		};
	}

	forwardConsole("log", trace);
	forwardConsole("debug", debug);
	forwardConsole("info", info);
	forwardConsole("warn", warn);
	forwardConsole("error", error);

	await attachConsole();

	const app = createApp(App);
	const pinia = createPinia();

	app.use(pinia);

	// Setup stores and dependency injection before mounting
	const settingsStore = useSettingsStore();
	const globalStore = useGlobalStore();

	// Provide real implementations for dependency injection
	await useAppProviders(app);

	// Initialize the app
	app.mount("#app");

	// Initialize error service after stores are ready
	// Set up error listener to show alerts to users
	ErrorHandler.addListener((error) => {
		// Map error severity to alert type
		let alertType: "error" | "warning" | "info" = "error";
		if (error.severity === ErrorSeverity.LOW) {
			alertType = "info";
		} else if (error.severity === ErrorSeverity.MEDIUM) {
			alertType = "warning";
		}

		// Show user-friendly alert
		globalStore.uiStore.addAlert({
			key: `error-${Date.now()}`, // Unique key based on timestamp
			type: alertType,
			title: error.category
				.replace(/_/g, " ")
				.toLowerCase()
				.replace(/\b\w/g, (l) => l.toUpperCase()),
			message: error.message,
			timeout: error.severity === ErrorSeverity.LOW ? 3000 : 5000,
		});
	});

	// Initialize hotkeys with default callbacks
	// These are the callbacks that will be used no matter the hotkey configuration
	settingsStore.initializeHotkeys({
		next_move: () =>
			globalStore.gamesStore.nextMove(globalStore.uiStore.activeBoardId),
		prev_move: () =>
			globalStore.gamesStore.previousMove(globalStore.uiStore.activeBoardId),
		goto_start: () =>
			globalStore.gamesStore.navigateToStart(globalStore.uiStore.activeBoardId),
		goto_end: () =>
			globalStore.gamesStore.navigateToEnd(globalStore.uiStore.activeBoardId),
		toggle_left_panel: () => globalStore.uiStore.toggleLeftPanel(),
		toggle_right_panel: () => globalStore.uiStore.toggleRightPanel(),
		open_settings: () => globalStore.uiStore.updateSettingsModalOpen(true),
	});
}

main();
