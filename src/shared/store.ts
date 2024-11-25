import { createGlobalState } from "@vueuse/core";
import { computed, ref } from "vue";
import { applicationLayout } from "../applicationLayout";
import api from "./api";
import {
  type IAPIGame,
  type ILayout,
  type IWindow,
  type IWindowContainer,
  validateWindow,
  validateWindowContainer,
} from "./types";

// Helper to get the visible windows from the layout and its children
function getVisibleWindowsHelper(layout: ILayout): IWindow[] {
  const visibleWindows: IWindow[] = [];

  // If it's a container, check that it's visible and not collapsed
  // and then recursively check its children
  if (validateWindowContainer(layout).success) {
    const container = layout as IWindowContainer;
    if (container.visible && !container.collapsed) {
      // biome-ignore lint/complexity/noForEach: TODO change later
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

function getDefaultTheme() {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

export const useGlobalState = createGlobalState(() => {
  // Global state
  const layout = ref<ILayout>(applicationLayout);
  const selectedGame = ref<IAPIGame | null>(null);
  const games = ref<IAPIGame[]>([]);
  const selectedGameLocation = ref<number | null>(0); // The move number of the selected game

  const UIState = ref<{
    visibleGameHeaders: string[];
    theme: "light" | "dark";
  }>({
    visibleGameHeaders: [],
    theme: getDefaultTheme(),
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
    value: string | number | boolean | null
  ) => {
    const window = getWindowById(layout.value, windowId);
    if (window && property in window) {
      // Use type assertion to IWindow since we know it's a window property
      (window as IWindow)[property as keyof IWindow] = value as never;
    }
  };

  const setSelectedGame = (newGame: IAPIGame | null) => {
    selectedGame.value = newGame;
    selectedGameLocation.value = 0;
  };

  const setSelectedGameLocation = (newLocation: number) => {
    selectedGameLocation.value = newLocation;
  };

  const toggleTheme = () => {
    setTheme(UIState.value.theme === "light" ? "dark" : "light");
  };

  const setTheme = (newTheme: "light" | "dark") => {
    if (newTheme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
    UIState.value.theme = newTheme;
  };
  setTheme(UIState.value.theme);

  // API Actions
  const updateGames = async () => {
    const parsedState = await api.getExplorerState();
    games.value = parsedState.games;

    const defaultHeaders = () => {
      const headers =
        selectedGame.value?.headers ?? games.value[0]?.headers ?? {};
      return Object.keys(headers).filter(
        (key) => headers[key] !== "" && !headers[key].includes("?")
      );
    };
    if (UIState.value.visibleGameHeaders.length === 0) {
      UIState.value.visibleGameHeaders = defaultHeaders();
    }
  };

  const fetchSelectedGame = async () => {
    const parsedGame = await api.getSelectedGame();
    selectedGame.value = parsedGame;
  };

  const parsePgnText = async (pgnText: string) => {
    await api.parsePgnText(pgnText);
    await updateGames();
    await fetchSelectedGame();
  };

  const emptyDatabase = async () => {
    await api.emptyDatabase();
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
    toggleTheme,
    // API Actions
    updateGames,
    fetchSelectedGame,
    parsePgnText,
    emptyDatabase,
  };
});
