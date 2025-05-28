import { defineStore } from "pinia";
import {
  type BoardTheme,
  BoardThemes,
  type DarkUITheme,
  type LightUITheme,
  type UITheme,
  UIThemes,
  darkUIThemes,
  lightUIThemes,
} from "../shared/themes";

// Layout configuration interface
interface LayoutConfig {
  leftPanelWidth: number;
  rightPanelWidth: number;
  enginePanelWidth: number;
  boardHeight: number;
  panelCollapsed: {
    left: boolean;
    right: boolean;
    engine: boolean;
  };
}

const defaultLightThemeKey = "defaultLightTheme";
const defaultDarkThemeKey = "defaultDarkTheme";
const layoutPreferencesKey = "layoutPreferences";

function getDefaultTheme(): UITheme {
  const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const defaultTheme = isDark
    ? localStorage.getItem(defaultDarkThemeKey) ?? "dark"
    : localStorage.getItem(defaultLightThemeKey) ?? "light";

  localStorage.setItem("theme", defaultTheme);
  document.documentElement.dataset.theme = defaultTheme;
  return defaultTheme as UITheme;
}

function getDefaultBoardTheme(): BoardTheme {
  const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  return isDark ? BoardThemes.dark : BoardThemes.light;
}

export const useUIStore = defineStore("ui", {
  state: () => ({
    // Existing state
    visibleGameHeaders: [
      "Event",
      "Date",
      "White",
      "Black",
      "Result",
      "Opening",
    ] as string[],

    theme: getDefaultTheme(),
    defaultLightTheme: (localStorage.getItem(defaultLightThemeKey) ??
      "light") as LightUITheme,
    defaultDarkTheme: (localStorage.getItem(defaultDarkThemeKey) ??
      "dark") as DarkUITheme,
    boardTheme: getDefaultBoardTheme(),

    boardSquareSize: 64,
    _whiteOnSide: "top" as "top" | "bottom",

    // Enhanced layout state
    layout: {
      leftPanelWidth: 300,
      rightPanelWidth: 250,
      enginePanelWidth: 400,
      boardHeight: 600,
      panelCollapsed: { left: false, right: false, engine: false },
    } as LayoutConfig,

    // Multi-board support
    activeBoardIds: [0] as number[],
    activeBoardId: 0,
    nextBoardId: 1,

    // Stacked panel states
    stackedPanelStates: {} as {
      [panelId: string]: {
        mode: "tabs" | "accordion" | "vertical";
        activeTab?: string;
        collapsedSections?: string[];
      };
    },

    // Panel visibility
    leftPanelOpen: true,
    rightPanelOpen: true,
    gameLibraryViewOpen: true,
    moveTreeViewOpen: true,
    engineViewOpen: false,

    // Game Library state
    gameLibraryView: "grid" as "grid" | "list",
    gameLibraryViewSortBy: "date" as
      | "date"
      | "event"
      | "white"
      | "black"
      | "result"
      | "opening",
    gameLibraryViewSortOrder: "desc" as "asc" | "desc",
    gameLibraryViewFilter: "all" as "all" | "favorites" | "tags",
    gameLibraryViewFilterTags: [] as string[],

    // Modal states
    settingsModalOpen: false,
  }),

  getters: {
    getLeftPanelOpen: (state) => state.leftPanelOpen,
    getRightPanelOpen: (state) => state.rightPanelOpen,
    getGameLibraryViewOpen: (state) => state.gameLibraryViewOpen,
    getMoveTreeViewOpen: (state) => state.moveTreeViewOpen,
    getEngineViewOpen: (state) => state.engineViewOpen,

    getTheme: (state) => state.theme,
    isDarkMode: (state) => darkUIThemes.includes(state.theme as DarkUITheme),
    getBoardTheme: (state) => state.boardTheme,
    getBoardSquareSize: (state) => state.boardSquareSize,
    whiteOnSide: (state) => state._whiteOnSide,

    getSettingsModalOpen: (state) => state.settingsModalOpen,

    // Multi-board getters
    getActiveBoardIds: (state) => state.activeBoardIds,
    getActiveBoardId: (state) => state.activeBoardId,

    // Responsive layout
    isSmallScreen: () => window.innerWidth < 768,
    isMediumScreen: () => window.innerWidth >= 768 && window.innerWidth < 1024,
    isLargeScreen: () => window.innerWidth >= 1024,

    // Panel state
    panelStates: (state) => ({
      gameLibrary: {
        visible: state.gameLibraryViewOpen,
        width: state.layout.leftPanelWidth,
        collapsed: state.layout.panelCollapsed.left,
      },
      moveTree: {
        visible: state.moveTreeViewOpen,
        width: state.layout.rightPanelWidth,
        collapsed: state.layout.panelCollapsed.right,
      },
      engine: {
        visible: state.engineViewOpen,
        width: state.layout.enginePanelWidth,
        collapsed: state.layout.panelCollapsed.engine,
      },
    }),
  },

  actions: {
    // Existing theme actions
    toggleTheme() {
      const isDark = darkUIThemes.includes(this.theme as DarkUITheme);
      const newTheme = isDark ? this.defaultLightTheme : this.defaultDarkTheme;
      this.setTheme(newTheme);
    },

    setTheme(theme: UITheme) {
      if (!UIThemes.includes(theme)) {
        throw new Error(`Invalid theme: ${theme}`);
      }
      this.theme = theme;
      localStorage.setItem("theme", theme);
      document.documentElement.dataset.theme = theme;

      const isDark = darkUIThemes.includes(theme as DarkUITheme);
      this.boardTheme = isDark ? BoardThemes.dark : BoardThemes.light;
    },

    setDefaultTheme(theme: LightUITheme | DarkUITheme) {
      const isDark = darkUIThemes.includes(theme as DarkUITheme);
      const type = isDark ? "dark" : "light";

      if (type === "light") {
        if (!lightUIThemes.includes(theme as LightUITheme)) {
          throw new Error(`Invalid light theme: ${theme}`);
        }
        this.defaultLightTheme = theme as LightUITheme;
      } else {
        if (!darkUIThemes.includes(theme as DarkUITheme)) {
          throw new Error(`Invalid dark theme: ${theme}`);
        }
        this.defaultDarkTheme = theme as DarkUITheme;
      }

      localStorage.setItem(
        type === "light" ? defaultLightThemeKey : defaultDarkThemeKey,
        theme
      );
    },

    // Enhanced layout actions
    updateLayoutDimension(dimension: keyof LayoutConfig, value: number) {
      if (dimension === "panelCollapsed") return; // Handle collapsed state separately

      // biome-ignore lint/suspicious/noExplicitAny: <explanation>
      (this.layout as any)[dimension] = value;
      this.saveLayoutPreferences();
    },

    togglePanelCollapse(panel: "left" | "right" | "engine") {
      this.layout.panelCollapsed[panel] = !this.layout.panelCollapsed[panel];
      this.saveLayoutPreferences();
    },

    updateBoardTheme(theme: BoardTheme) {
      this.boardTheme = theme;
    },

    updateBoardSquareSize(size: number) {
      this.boardSquareSize = size;
    },

    setWhiteOnSide(side?: "top" | "bottom") {
      this._whiteOnSide =
        side ?? (this._whiteOnSide === "top" ? "bottom" : "top");
    },

    // Multi-board management
    createNewBoard(): number {
      const newBoardId = this.nextBoardId++;
      this.activeBoardIds.push(newBoardId);
      this.setActiveBoardId(newBoardId);
      return newBoardId;
    },

    setActiveBoardId(boardId: number) {
      if (this.activeBoardIds.includes(boardId)) {
        this.activeBoardId = boardId;
      }
    },

    closeBoardTab(boardId: number) {
      const index = this.activeBoardIds.indexOf(boardId);
      if (index === -1) return;

      this.activeBoardIds.splice(index, 1);

      // Switch to another board if closing the active one
      if (this.activeBoardId === boardId && this.activeBoardIds.length > 0) {
        this.activeBoardId = this.activeBoardIds[Math.max(0, index - 1)];
      }

      // Ensure at least one board exists
      if (this.activeBoardIds.length === 0) {
        this.activeBoardIds.push(0);
        this.activeBoardId = 0;
      }
    },

    // Panel visibility
    updateSettingsModalOpen(open?: boolean) {
      this.settingsModalOpen = open ?? !this.settingsModalOpen;
    },

    toggleLeftPanel() {
      this.leftPanelOpen = !this.leftPanelOpen;
    },

    toggleRightPanel() {
      this.rightPanelOpen = !this.rightPanelOpen;
    },

    toggleGameLibraryView() {
      this.gameLibraryViewOpen = !this.gameLibraryViewOpen;
    },

    toggleMoveTreeView() {
      this.moveTreeViewOpen = !this.moveTreeViewOpen;
    },

    toggleEngineView() {
      this.engineViewOpen = !this.engineViewOpen;
    },

    // Game library actions
    gameLibraryViewUpdateSortBy(
      sortBy: "date" | "event" | "white" | "black" | "result" | "opening"
    ) {
      this.gameLibraryViewSortBy = sortBy;
    },

    gameLibraryViewUpdateSortOrder(sortOrder: "asc" | "desc") {
      this.gameLibraryViewSortOrder = sortOrder;
    },

    gameLibraryViewUpdateFilter(filter: "all" | "favorites" | "tags") {
      this.gameLibraryViewFilter = filter;
    },

    gameLibraryViewUpdateFilterTags(tags: string[]) {
      this.gameLibraryViewFilterTags = tags;
    },

    gameLibraryViewUpdateView(view: "grid" | "list") {
      this.gameLibraryView = view;
    },

    // Responsive layout actions
    applyResponsiveLayout() {
      if (this.isSmallScreen) {
        // Mobile: Stack panels vertically, collapse by default
        this.layout.panelCollapsed.left = true;
        this.layout.panelCollapsed.right = true;
        this.layout.leftPanelWidth = Math.min(this.layout.leftPanelWidth, 250);
        this.layout.rightPanelWidth = Math.min(
          this.layout.rightPanelWidth,
          250
        );
      } else if (this.isMediumScreen) {
        // Tablet: Reduce panel sizes
        this.layout.leftPanelWidth = Math.min(this.layout.leftPanelWidth, 300);
        this.layout.rightPanelWidth = Math.min(
          this.layout.rightPanelWidth,
          280
        );
        this.layout.enginePanelWidth = Math.min(
          this.layout.enginePanelWidth,
          350
        );
      }
      // Large screen: Use full layout
    },

    // Persistence
    saveLayoutPreferences() {
      const preferences = {
        layout: this.layout,
        panelVisibility: {
          gameLibrary: this.gameLibraryViewOpen,
          moveTree: this.moveTreeViewOpen,
          engine: this.engineViewOpen,
        },
        activeBoardIds: this.activeBoardIds,
        activeBoardId: this.activeBoardId,
        stackedPanelStates: this.stackedPanelStates,
      };

      localStorage.setItem(layoutPreferencesKey, JSON.stringify(preferences));
    },

    loadLayoutPreferences() {
      try {
        const saved = localStorage.getItem(layoutPreferencesKey);
        if (!saved) return;

        const preferences = JSON.parse(saved);

        // Apply saved layout
        if (preferences.layout) {
          this.layout = { ...this.layout, ...preferences.layout };
        }

        // Apply panel visibility
        if (preferences.panelVisibility) {
          this.gameLibraryViewOpen =
            preferences.panelVisibility.gameLibrary ?? true;
          this.moveTreeViewOpen = preferences.panelVisibility.moveTree ?? true;
          this.engineViewOpen = preferences.panelVisibility.engine ?? false;
        }

        // Apply multi-board state
        if (
          preferences.activeBoardIds &&
          preferences.activeBoardIds.length > 0
        ) {
          this.activeBoardIds = preferences.activeBoardIds;
          this.activeBoardId =
            preferences.activeBoardId ?? preferences.activeBoardIds[0];
          this.nextBoardId = Math.max(...preferences.activeBoardIds) + 1;
        }

        // Apply stacked panel states
        if (preferences.stackedPanelStates) {
          this.stackedPanelStates = preferences.stackedPanelStates;
        }
      } catch (error) {
        console.warn("Failed to load layout preferences:", error);
      }
    },

    // Reset to defaults
    resetLayoutToDefaults() {
      this.layout = {
        leftPanelWidth: 300,
        rightPanelWidth: 250,
        enginePanelWidth: 400,
        boardHeight: 600,
        panelCollapsed: { left: false, right: false, engine: false },
      };
      localStorage.removeItem(layoutPreferencesKey);
    },

    // Stacked panel management
    updateStackedPanelState(
      panelId: string,
      updates: Partial<{
        mode: "tabs" | "accordion" | "vertical";
        activeTab: string;
        collapsedSections: string[];
      }>
    ) {
      if (!this.stackedPanelStates[panelId]) {
        this.stackedPanelStates[panelId] = {
          mode: "accordion",
          collapsedSections: [],
        };
      }

      Object.assign(this.stackedPanelStates[panelId], updates);
      this.saveLayoutPreferences();
    },

    setStackedPanelActiveTab(panelId: string, tabId: string) {
      this.updateStackedPanelState(panelId, { activeTab: tabId });
    },

    toggleStackedPanelSection(panelId: string, sectionId: string) {
      const panelState = this.stackedPanelStates[panelId];
      if (!panelState) {
        this.updateStackedPanelState(panelId, {
          collapsedSections: [sectionId],
        });
        return;
      }

      const collapsedSections = panelState.collapsedSections || [];
      const isCollapsed = collapsedSections.includes(sectionId);

      if (isCollapsed) {
        // Expand section
        this.updateStackedPanelState(panelId, {
          collapsedSections: collapsedSections.filter((id) => id !== sectionId),
        });
      } else {
        // Collapse section
        this.updateStackedPanelState(panelId, {
          collapsedSections: [...collapsedSections, sectionId],
        });
      }
    },

    getStackedPanelState(panelId: string) {
      return (
        this.stackedPanelStates[panelId] || {
          mode: "accordion" as const,
          collapsedSections: [],
        }
      );
    },
  },
});
