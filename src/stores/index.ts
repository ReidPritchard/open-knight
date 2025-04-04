import { defineStore } from "pinia";
import api from "../shared/api";
import type { ExplorerGame } from "../shared/types";
import { useGamesStore } from "./games";
import { useSettingsStore } from "./settings";
import { useUIStore } from "./ui";

export const useGlobalStore = defineStore("global", {
  state: () => ({
    internalGamesStore: useGamesStore(),
    internalUiStore: useUIStore(),
    internalSettingsStore: useSettingsStore(),
    explorer: {
      games: [] as ExplorerGame[],
    },
  }),
  getters: {
    gamesStore: (state) => state.internalGamesStore,
    uiStore: (state) => state.internalUiStore,
    settingsStore: (state) => state.internalSettingsStore,
    explorerGames: (state) => state.explorer.games,
  },
  actions: {
    async fetchExplorerGames() {
      const games = await api.games.GET.explorer();
      this.explorer.games = games;
      return games;
    },
    async importDemoGames() {
      await api.games.POST.importDemoGames();
      await this.fetchExplorerGames();
    },
    async resetDatabase() {
      await api.emptyDatabase();
      await this.fetchExplorerGames();
    },
  },
});
