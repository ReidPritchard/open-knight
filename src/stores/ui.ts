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

// Workspace configuration
interface Workspace {
  id: string;
  name: string;
  layout: LayoutConfig;
  panelVisibility: {
    gameLibrary: boolean;
    moveTree: boolean;
    engine: boolean;
  };
  description?: string;
}

// Default workspaces
const DEFAULT_WORKSPACES: Workspace[] = [
  {
    id: "analysis",
    name: "Analysis",
    description: "Full analysis setup with all panels",
    layout: {
      leftPanelWidth: 300,
      rightPanelWidth: 250,
      enginePanelWidth: 400,
      boardHeight: 600,
      panelCollapsed: { left: false, right: false, engine: false },
    },
    panelVisibility: {
      gameLibrary: true,
      moveTree: true,
      engine: true,
    },
  },
  {
    id: "study",
    name: "Study",
    description: "Game library and move tree for studying",
    layout: {
      leftPanelWidth: 350,
      rightPanelWidth: 300,
      enginePanelWidth: 400,
      boardHeight: 600,
      panelCollapsed: { left: false, right: false, engine: true },
    },
    panelVisibility: {
      gameLibrary: true,
      moveTree: true,
      engine: false,
    },
  },
  {
    id: "minimal",
    name: "Minimal",
    description: "Just the chess board for focused play",
    layout: {
      leftPanelWidth: 300,
      rightPanelWidth: 250,
      enginePanelWidth: 400,
      boardHeight: 600,
      panelCollapsed: { left: true, right: true, engine: true },
    },
    panelVisibility: {
      gameLibrary: false,
      moveTree: false,
      engine: false,
    },
  },
];

const defaultLightThemeKey = "defaultLightTheme";
const defaultDarkThemeKey = "defaultDarkTheme";
const layoutPreferencesKey = "layoutPreferences";
const workspaceKey = "activeWorkspace";

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

    // Workspace management
    workspaces: DEFAULT_WORKSPACES,
    currentWorkspaceId: "analysis",
    customWorkspaces: [] as Workspace[],

    // Multi-board support
    activeBoardIds: [0] as number[],
    activeBoardId: 0,
    nextBoardId: 1,

    // Panel visibility (legacy - now part of workspace)
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
    // Existing getters
    getGameLibraryViewOpen: (state) => state.gameLibraryViewOpen,
    getMoveTreeViewOpen: (state) => state.moveTreeViewOpen,
    getEngineViewOpen: (state) => state.engineViewOpen,
    getBoardTheme: (state) => state.boardTheme,
    getBoardSquareSize: (state) => state.boardSquareSize,
    whiteOnSide: (state) => state._whiteOnSide,
    getSettingsModalOpen: (state) => state.settingsModalOpen,

    // Enhanced layout getters
    currentWorkspace: (state): Workspace | undefined => {
      return [...state.workspaces, ...state.customWorkspaces].find(
        (w) => w.id === state.currentWorkspaceId
      );
    },

    availableWorkspaces: (state) => [
      ...state.workspaces,
      ...state.customWorkspaces,
    ],

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

    // Workspace management
    switchWorkspace(workspaceId: string) {
      const workspace = this.availableWorkspaces.find(
        (w) => w.id === workspaceId
      );
      if (!workspace) return;

      this.currentWorkspaceId = workspaceId;
      this.layout = { ...workspace.layout };

      // Apply panel visibility
      this.gameLibraryViewOpen = workspace.panelVisibility.gameLibrary;
      this.moveTreeViewOpen = workspace.panelVisibility.moveTree;
      this.engineViewOpen = workspace.panelVisibility.engine;

      localStorage.setItem(workspaceKey, workspaceId);
      this.saveLayoutPreferences();
    },

    createCustomWorkspace(name: string, description?: string) {
      const newWorkspace: Workspace = {
        id: `custom_${Date.now()}`,
        name,
        description,
        layout: { ...this.layout },
        panelVisibility: {
          gameLibrary: this.gameLibraryViewOpen,
          moveTree: this.moveTreeViewOpen,
          engine: this.engineViewOpen,
        },
      };

      this.customWorkspaces.push(newWorkspace);
      this.saveLayoutPreferences();
      return newWorkspace.id;
    },

    deleteCustomWorkspace(workspaceId: string) {
      const index = this.customWorkspaces.findIndex(
        (w) => w.id === workspaceId
      );
      if (index !== -1) {
        this.customWorkspaces.splice(index, 1);

        // Switch to default workspace if deleting current one
        if (this.currentWorkspaceId === workspaceId) {
          this.switchWorkspace("analysis");
        }

        this.saveLayoutPreferences();
      }
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

    // Panel visibility (legacy support)
    updateSettingsModalOpen(open?: boolean) {
      this.settingsModalOpen = open ?? !this.settingsModalOpen;
    },

    toggleGameLibraryView() {
      this.gameLibraryViewOpen = !this.gameLibraryViewOpen;
      this.updateCurrentWorkspaceVisibility();
    },

    toggleMoveTreeView() {
      this.moveTreeViewOpen = !this.moveTreeViewOpen;
      this.updateCurrentWorkspaceVisibility();
    },

    toggleEngineView() {
      this.engineViewOpen = !this.engineViewOpen;
      this.updateCurrentWorkspaceVisibility();
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
        currentWorkspaceId: this.currentWorkspaceId,
        customWorkspaces: this.customWorkspaces,
        panelVisibility: {
          gameLibrary: this.gameLibraryViewOpen,
          moveTree: this.moveTreeViewOpen,
          engine: this.engineViewOpen,
        },
        activeBoardIds: this.activeBoardIds,
        activeBoardId: this.activeBoardId,
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

        // Apply workspace
        if (preferences.currentWorkspaceId) {
          const workspace = this.availableWorkspaces.find(
            (w) => w.id === preferences.currentWorkspaceId
          );
          if (workspace) {
            this.currentWorkspaceId = preferences.currentWorkspaceId;
          }
        }

        // Apply custom workspaces
        if (preferences.customWorkspaces) {
          this.customWorkspaces = preferences.customWorkspaces;
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
      } catch (error) {
        console.warn("Failed to load layout preferences:", error);
      }
    },

    // Helper to update current workspace when panels change
    updateCurrentWorkspaceVisibility() {
      const currentWorkspace = this.currentWorkspace;
      if (currentWorkspace?.id?.startsWith("custom_")) {
        // Update custom workspace
        currentWorkspace.panelVisibility = {
          gameLibrary: this.gameLibraryViewOpen,
          moveTree: this.moveTreeViewOpen,
          engine: this.engineViewOpen,
        };
        this.saveLayoutPreferences();
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
      this.switchWorkspace("analysis");
      localStorage.removeItem(layoutPreferencesKey);
    },
  },
});
