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

const defaultLightThemeKey = "defaultLightTheme";
const defaultDarkThemeKey = "defaultDarkTheme";

function getDefaultTheme(): UITheme {
	const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;

	const defaultTheme = isDark
		? (localStorage.getItem(defaultDarkThemeKey) ?? "dark")
		: (localStorage.getItem(defaultLightThemeKey) ?? "light");

	// Set the theme in localStorage
	localStorage.setItem("theme", defaultTheme);

	// Update the document class
	document.documentElement.dataset.theme = defaultTheme;

	return defaultTheme as UITheme;
}

function getDefaultBoardTheme(): BoardTheme {
	const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
	return isDark ? BoardThemes.dark : BoardThemes.light;
}

export const useUIStore = defineStore("ui", {
	state: () => ({
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

		/**
		 * The orientation of the board, by which side white is playing from
		 * TODO: Might want to make this per-game board rather than global
		 * might be ok for now though
		 *
		 * FIXME: Currently this is backwards ("top" means white on bottom)
		 * either rename this or invert the logic
		 */
		_whiteOnSide: "top" as "top" | "bottom",

		/**
		 * Game Library/Explorer view
		 */
		gameLibraryViewOpen: false,
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

		moveTreeViewOpen: true,

		engineViewOpen: false,

		/**
		 * Settings Modal
		 */
		settingsModalOpen: false,
	}),

	getters: {
		getGameLibraryViewOpen: (state) => state.gameLibraryViewOpen,
		getMoveTreeViewOpen: (state) => state.moveTreeViewOpen,
		getEngineViewOpen: (state) => state.engineViewOpen,

		getBoardTheme: (state) => state.boardTheme,
		getBoardSquareSize: (state) => state.boardSquareSize,
		whiteOnSide: (state) => state._whiteOnSide,

		getGameLibraryViewSortByOptions: () => [
			"date",
			"event",
			"white",
			"black",
			"result",
			"opening",
		],
		getGameLibraryViewSortBy: (state) => state.gameLibraryViewSortBy,
		getGameLibraryViewSortOrderOptions: () => ["asc", "desc"],
		getGameLibraryViewSortOrder: (state) => state.gameLibraryViewSortOrder,
		getGameLibraryViewFilterOptions: () => ["all", "favorites", "tags"],
		getGameLibraryViewFilter: (state) => state.gameLibraryViewFilter,
		getGameLibraryViewFilterTags: (state) => state.gameLibraryViewFilterTags,

		getSettingsModalOpen: (state) => state.settingsModalOpen,
	},

	actions: {
		/**
		 * Toggle between light and dark theme
		 *
		 * The theme set is the default light/dark theme
		 */
		toggleTheme() {
			// Check if the current theme is light or dark
			const isDark = darkUIThemes.includes(this.theme as DarkUITheme);

			// Toggle the theme
			const newTheme = isDark ? this.defaultLightTheme : this.defaultDarkTheme;

			this.setTheme(newTheme);
		},

		setTheme(theme: UITheme) {
			if (!UIThemes.includes(theme)) {
				throw new Error(`Invalid theme: ${theme}`);
			}

			this.theme = theme;

			// Set the theme in localStorage
			localStorage.setItem("theme", theme);

			// Update the DOM
			document.documentElement.dataset.theme = theme;

			// Update the board theme
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

			// Save the default theme to localStorage
			localStorage.setItem(
				type === "light" ? defaultLightThemeKey : defaultDarkThemeKey,
				theme,
			);
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

		updateSettingsModalOpen(open?: boolean) {
			this.settingsModalOpen = open ?? !this.settingsModalOpen;
		},

		toggleGameLibraryView() {
			this.gameLibraryViewOpen = !this.gameLibraryViewOpen;
		},

		toggleMoveTreeView() {
			this.moveTreeViewOpen = !this.moveTreeViewOpen;
		},

		toggleEngineView() {
			this.engineViewOpen = !this.engineViewOpen;
			if (this.engineViewOpen) {
				this.moveTreeViewOpen = false;
				this.gameLibraryViewOpen = false;
			}
		},

		gameLibraryViewUpdateSortBy(
			sortBy: "date" | "event" | "white" | "black" | "result" | "opening",
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
	},
});
