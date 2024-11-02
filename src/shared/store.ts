import { createGlobalState } from "@vueuse/core";
import { ref, computed } from "vue";
import { applicationLayout } from "../applicationLayout";
import {
  ILayout,
  IWindowContainer,
  validateWindowContainer,
  validateWindow,
  IWindow,
  IGame,
} from "./types";
import { invoke } from "@tauri-apps/api/core";
import {
  apiExplorerStateToExplorerState,
  apiSelectedGameToGame,
} from "./api-conversions";

// Helper to get the visible windows from the layout and its children
function getVisibleWindowsHelper(layout: ILayout): IWindow[] {
  const visibleWindows: IWindow[] = [];

  // If it's a container, check that it's visible and not collapsed
  // and then recursively check its children
  if (validateWindowContainer(layout).success) {
    const container = layout as IWindowContainer;
    if (container.visible && !container.collapsed) {
      container.children.forEach((child) => {
        const visibleChildWindows = getVisibleWindowsHelper(child);
        visibleWindows.push(...visibleChildWindows);
      });
    }
  }

  // If it's a window, check that it's visible and not collapsed
  if (validateWindow(layout).success) {
    const window = layout as IWindow;
    if (window.visible && !window.collapsed) {
      visibleWindows.push(window);
    }
  }

  return visibleWindows;
}

function getWindowById(layout: ILayout, windowId: string): IWindow | null {
  if (validateWindowContainer(layout).success) {
    const container = layout as IWindowContainer;
    // recursively search through the container's children (make sure to include panels)
    for (const child of container.children) {
      const result = getWindowById(child, windowId);
      if (result) {
        return result;
      }
    }

    // TODO: Check the panels (use validateSimpleContainer)
  }

  return null;
}

export const useGlobalState = createGlobalState(() => {
  // Global state
  const layout = ref<ILayout>(applicationLayout);
  const selectedGame = ref<IGame | null>(null);
  const games = ref<IGame[]>([]);
  const selectedGameLocation = ref<number | null>(0); // The move number of the selected game

  const UIState = ref<{
    visibleGameHeaders: string[];
  }>({
    visibleGameHeaders: [],
  });

  // Getters (used to compute derived state for specific components)
  const visibleWindows = computed(() => getVisibleWindowsHelper(layout.value));

  // Actions
  const setLayout = (newLayout: ILayout) => {
    layout.value = newLayout;
  };

  const updateWindowProperty = (
    windowId: string,
    property: string,
    value: any,
  ) => {
    const window = getWindowById(layout.value, windowId);
    if (window) {
      (window as any)[property] = value;
    }
  };

  const setSelectedGame = (newGame: IGame | null) => {
    selectedGame.value = newGame;
    selectedGameLocation.value = 0;
  };

  const setSelectedGameLocation = (newLocation: number) => {
    selectedGameLocation.value = newLocation;
  };

  // API Actions
  const updateGames = async () => {
    const state: string = await invoke("get_explorer_state");
    const parsedState = apiExplorerStateToExplorerState(state);
    console.log(parsedState);
    games.value = parsedState.games;

    const defaultHeaders = () => {
      const headers =
        selectedGame.value?.headers ?? games.value[0]?.headers ?? {};
      return Object.keys(headers).filter(
        (key) => headers[key] !== "" && !headers[key].includes("?"),
      );
    };
    if (UIState.value.visibleGameHeaders.length === 0) {
      UIState.value.visibleGameHeaders = defaultHeaders();
    }
  };

  const fetchSelectedGame = async () => {
    const response: string = await invoke("get_selected_game");
    const game: string | null = response === "null" ? null : response;
    const parsedGame = apiSelectedGameToGame(game);
    selectedGame.value = parsedGame;
  };

  const parsePgnText = async (pgnText: string) => {
    await invoke("parse_pgn", { pgn: pgnText });
    await updateGames();
    await fetchSelectedGame();
  };

  const emptyDatabase = async () => {
    await invoke("empty_db");
    await updateGames();
  };

  return {
    // State
    layout,
    selectedGame,
    games,
    selectedGameLocation,
    UIState,
    // Getters
    visibleWindows,
    // Actions
    setLayout,
    updateWindowProperty,
    setSelectedGame,
    setSelectedGameLocation,
    // API Actions
    updateGames,
    fetchSelectedGame,
    parsePgnText,
    emptyDatabase,
  };
});
