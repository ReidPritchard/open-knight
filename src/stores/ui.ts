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
     */
    boardWhiteOrientation: "bottom" as "top" | "bottom",
  }),

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
  },
});
