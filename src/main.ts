import { createPinia } from "pinia";

import "./style.css";

import { createApp } from "vue";
import App from "./App.vue";
import { ErrorHandler, ErrorSeverity } from "./services/ErrorService";
import { useGlobalStore } from "./stores";
import { useSettingsStore } from "./stores/settings";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);

// Initialize the app
app.mount("#app");

// Setup hotkeys after app is mounted to ensure stores are ready
const settingsStore = useSettingsStore();
const globalStore = useGlobalStore();

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
