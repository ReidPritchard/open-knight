import { defineStore } from "pinia";

function getDefaultTheme(): "light" | "dark" {
  const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;

  // Set the theme in localStorage
  localStorage.setItem("theme", isDark ? "dark" : "light");
  // Set the class on the document element
  if (isDark) {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }

  return isDark ? "dark" : "light";
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
    theme: getDefaultTheme() as "light" | "dark",
    /**
     * The orientation of the board, by which side white is playing from
     * TODO: Might want to make this per-game board rather than global
     * might be ok for now though
     */
    boardWhiteOrientation: "bottom" as "top" | "bottom",

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
  }),

  getters: {
    getGameLibraryViewOpen: (state) => state.gameLibraryViewOpen,
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
  },

  actions: {
    toggleTheme() {
      const newTheme = this.theme === "light" ? "dark" : "light";

      // Set the theme in localStorage
      localStorage.setItem("theme", newTheme);
      // Set the class on the document element
      if (newTheme === "dark") {
        document.documentElement.classList.add("dark");
      } else {
        document.documentElement.classList.remove("dark");
      }

      this.theme = newTheme;
    },

    toggleGameLibraryView() {
      this.gameLibraryViewOpen = !this.gameLibraryViewOpen;
    },

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
  },
});
