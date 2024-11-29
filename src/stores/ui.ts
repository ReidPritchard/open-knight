import { defineStore } from "pinia";
import { applicationLayout } from "../applicationLayout";
import type { ILayout, IWindow } from "../shared/types";

function getDefaultTheme() {
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

function getWindowById(layout: ILayout, windowId: string): IWindow | null {
  if ("children" in layout) {
    for (const child of layout.children) {
      const result = getWindowById(child, windowId);
      if (result) return result;
    }
  }
  return layout.id === windowId ? (layout as IWindow) : null;
}

export const useUIStore = defineStore("ui", {
  state: () => ({
    layout: applicationLayout as ILayout,
    visibleGameHeaders: [
      "Event",
      "Date",
      "White",
      "Black",
      "Result",
      "Opening",
    ] as string[],
    theme: getDefaultTheme() as "light" | "dark",
  }),

  actions: {
    setLayout(newLayout: ILayout) {
      this.layout = newLayout;
    },

    updateWindowProperty(
      windowId: string,
      property: string,
      value: string | number | boolean | null
    ) {
      const window = getWindowById(this.layout, windowId);
      if (window && property in window) {
        window[property as keyof typeof window] = value as never;
      }
    },

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
