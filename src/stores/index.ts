import { defineStore } from "pinia";
import { useGamesStore } from "./games";
import { useUIStore } from "./ui";
import type { ExplorerGame } from "../shared/types";
import api from "../shared/api";

export const useGlobalStore = defineStore("global", {
  state: () => ({
    gamesStore: useGamesStore(),
    uiStore: useUIStore(),

    explorer: {
      games: [] as ExplorerGame[],
    },
  }),
  actions: {
    games: {
      get: {
        async explorer() {
          const games = await api.games.GET.explorer();
          this.explorer.games = games;
          return games;
        },
      },
      post: {
        async importDemoGames() {
          await api.games.POST.importDemoGames();
          this.games.get.explorer();
        },
      },
    },
  },
});
