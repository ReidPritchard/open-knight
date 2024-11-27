import { defineStore } from "pinia";
import { useGameStore } from "./game";
import { useUIStore } from "./ui";

export const useGlobalStore = defineStore("global", {
  state: () => ({
    game: useGameStore(),
    ui: useUIStore(),
  }),
});
