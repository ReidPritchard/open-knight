<template>
  <Menubar :model="items">
    <template #start>
      <img alt="logo" src="../assets/logo.png" height="40" class="mr-2" />
    </template>
  </Menubar>
</template>

<script setup lang="ts">
import Menubar from "primevue/menubar";
import { MenuItem, MenuItemCommandEvent } from "primevue/menuitem";
import { computed } from "vue";
import { useGlobalState } from "../shared/store";
import { useDialog } from "primevue/usedialog";
import PgnImport from "./PgnImport.vue";

// Get the layout from the store
const { visibleWindows, emptyDatabase, toggleTheme, UIState } =
  useGlobalState();

const dialog = useDialog();

const onCommand = (event: MenuItemCommandEvent) => {
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
      emptyDatabase();
      break;
    case "Quit":
      // TODO: Implement a modal with a confirmation prompt
      break;
  }
};

const onViewCommand = (event: MenuItemCommandEvent) => {
  switch (event.item.label) {
    case themeLabel.value:
      toggleTheme();
      break;
  }
};

const theme = computed(() => UIState.value.theme);
const themeLabel = computed(() =>
  theme.value === "light" ? "Dark Mode" : "Light Mode",
);
const themeIcon = computed(() =>
  theme.value === "light" ? "pi pi-fw pi-moon" : "pi pi-fw pi-sun",
);

// Helper function to get window icon
const getWindowIcon = (id: string, fallback: string) =>
  visibleWindows.value?.find((child) => child.id === id)?.icon || fallback;

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
  } as MenuItem,
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
  } as MenuItem,
]);
</script>

<style scoped>
.menu-bar {
  background-color: var(--p-content-background);
}
</style>
