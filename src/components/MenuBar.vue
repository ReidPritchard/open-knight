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
import { computed, ref } from "vue";
import { useGlobalState } from "../shared/store";

// Get the layout from the store
const { getVisibleWindows } = useGlobalState();

const onCommand = (event: MenuItemCommandEvent) => {
  console.log(event);

  switch (event.item.label) {
    case "Load PGN":
      break;
    case "Export PGN":
      break;
    case "Quit":
      // window.close();
      break;
  }
};

const onViewCommand = (event: MenuItemCommandEvent) => {
  console.log(event);
};

const openWindows = computed(() => getVisibleWindows.value);

const items = ref([
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
      {
        label: "Quit",
        icon: "pi pi-fw pi-power-off",
        command: onCommand,
      },
    ],
  } as MenuItem,
  {
    label: "View",
    icon: "pi pi-fw pi-eye",
    visible: true,
    items: [
      {
        label: "Board",
        icon:
          openWindows.value?.find((child) => child.id === "board")?.icon ||
          "pi pi-fw pi-board",
        command: onViewCommand,
      },
      {
        label: "Game",
        icon:
          openWindows.value?.find((child) => child.id === "game")?.icon ||
          "pi pi-fw pi-game",
        command: onViewCommand,
      },
      {
        label: "Analysis",
        icon:
          openWindows.value?.find((child) => child.id === "analysis")?.icon ||
          "pi pi-fw pi-analysis",
        command: onViewCommand,
      },
      {
        label: "Settings",
        icon:
          openWindows.value?.find((child) => child.id === "settings")?.icon ||
          "pi pi-fw pi-settings",
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
