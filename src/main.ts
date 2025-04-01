import { createPinia } from "pinia";

import "./style.css";

import { createApp } from "vue";
import App from "./App.vue";
import { useSettingsStore } from "./stores/settings";
import { useGlobalStore } from "./stores";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);

// Initialize the app
app.mount("#app");

// Setup hotkeys after app is mounted to ensure stores are ready
const settingsStore = useSettingsStore();
const globalStore = useGlobalStore();

// Initialize hotkeys with default callbacks
// These are the callbacks that will be used no matter the hotkey configuration
settingsStore.initializeHotkeys({
  next_move: () => globalStore.gamesStore.nextMove(0), // Using board ID 0 as default
  prev_move: () => globalStore.gamesStore.previousMove(0),
  toggle_game_library: () => globalStore.uiStore.toggleGameLibraryView(),
  open_settings: () => globalStore.uiStore.updateSettingsModalOpen(true),
});
