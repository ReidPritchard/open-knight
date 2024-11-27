<template>
  <nav class="bg-white dark:bg-gray-800 border-b shadow-sm">
    <div class="flex flex-row items-center px-4 h-14">
      <!-- Logo section -->
      <img alt="logo" src="../assets/logo.png" height="40" class="h-10 mr-4" />

      <!-- Menu items -->
      <div class="flex flex-row space-x-2">
        <div
          v-for="menuItem in items"
          :key="menuItem.label"
          class="relative group"
        >
          <button
            class="px-3 py-2 rounded-md text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            @click="toggleMenu(menuItem)"
            v-if="menuItem.visible"
          >
            <span class="flex items-center">
              <i :class="menuItem.icon" class="mr-2"></i>
              {{ menuItem.label }}
            </span>
          </button>

          <!-- Dropdown menu -->
          <div
            v-if="menuItem.items && activeMenu === menuItem.label"
            class="absolute left-0 mt-1 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 z-50"
          >
            <div class="py-1">
              <button
                v-for="subItem in menuItem.items"
                :key="subItem.label"
                @click="handleMenuClick(subItem)"
                class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center"
              >
                <i :class="subItem.icon" class="mr-2"></i>
                {{ subItem.label }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { useDialog } from "primevue/usedialog";
import { computed, ref } from "vue";
import api from "../shared/api";
import { useGlobalStore } from "../stores";
import PgnImport from "./PgnImport.vue";

// Get the layout from the store
const { ui } = useGlobalStore();

const dialog = useDialog();

// Track active menu for dropdowns
const activeMenu = ref<string | null>(null);

const toggleMenu = (menuItem: { label: string }) => {
  activeMenu.value =
    activeMenu.value === menuItem.label ? null : menuItem.label;
};

// Click outside handler
const closeMenus = () => {
  activeMenu.value = null;
};

// Add click outside listener
if (typeof window !== "undefined") {
  window.addEventListener("click", (e) => {
    const target = e.target as HTMLElement;
    if (!target.closest(".group")) {
      closeMenus();
    }
  });
}

const handleMenuClick = (item: {
  label: string;
  command?: (event: { item: { label: string } }) => void;
}) => {
  if (item.command) {
    item.command({ item });
  }
  closeMenus();
};

const onCommand = (event: { item: { label: string } }) => {
  switch (event.item.label) {
    case "Load PGN":
      dialog.open(PgnImport, {
        props: {
          header: "Load PGN",
          style: {
            width: "50vw",
          },
          breakpoints: {
            "960px": "75vw",
            "640px": "90vw",
          },
          modal: true,
        },
      });
      break;
    case "Export PGN":
      // TODO: Implement a modal with a text area or file output
      break;
    case "Empty DB":
      api.emptyDatabase();
      break;
    case "Quit":
      // TODO: Implement a modal with a confirmation prompt
      break;
  }
};

const onViewCommand = (event: { item: { label: string } }) => {
  switch (event.item.label) {
    case themeLabel.value:
      ui.toggleTheme();
      break;
  }
};

const theme = computed(() => ui.$state.theme);
const themeLabel = computed(() =>
  theme.value === "light" ? "Dark Mode" : "Light Mode"
);
const themeIcon = computed(() =>
  theme.value === "light" ? "pi pi-fw pi-moon" : "pi pi-fw pi-sun"
);

// Helper function to get window icon
const getWindowIcon = (_id: string, fallback: string) => fallback;

const items = computed(() => [
  {
    label: "File",
    icon: "pi pi-fw pi-file",
    visible: true,
    items: [
      {
        label: "Load PGN",
        icon: "pi pi-fw pi-file-import",
        command: onCommand,
      },
      {
        label: "Export PGN",
        icon: "pi pi-fw pi-file-export",
        command: onCommand,
      },
      { label: "Empty DB", icon: "pi pi-fw pi-database", command: onCommand },
      { label: "Quit", icon: "pi pi-fw pi-power-off", command: onCommand },
    ],
  },
  {
    label: "View",
    icon: "pi pi-fw pi-eye",
    visible: true,
    items: [
      {
        label: themeLabel.value,
        icon: themeIcon.value,
        command: onViewCommand,
      },
      {
        label: "Board",
        icon: getWindowIcon("board", "pi pi-fw pi-board"),
        command: onViewCommand,
      },
      {
        label: "Game",
        icon: getWindowIcon("game", "pi pi-fw pi-game"),
        command: onViewCommand,
      },
      {
        label: "Analysis",
        icon: getWindowIcon("analysis", "pi pi-fw pi-analysis"),
        command: onViewCommand,
      },
      {
        label: "Settings",
        icon: getWindowIcon("settings", "pi pi-fw pi-settings"),
        command: onViewCommand,
      },
    ],
  },
]);
</script>

<style scoped>
.menu-bar {
  background-color: var(--p-content-background);
}
</style>
