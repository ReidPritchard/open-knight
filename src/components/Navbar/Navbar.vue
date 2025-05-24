<template>
  <header>
    <div class="navbar bg-base-200 text-base-content px-4 select-none">
      <!-- Left: App Title -->
      <div class="navbar-start">
        <h1 class="text-xl font-bold text-base-content cursor-default">
          Open Knight
        </h1>
      </div>

      <!-- Center: Main Toolbar -->
      <div class="navbar-center gap-1">
        <!-- New Game -->
        <button
          class="btn btn-sm tooltip tooltip-bottom"
          data-tip="New Game"
          @click="newGameClick"
        >
          <PhPlus class="w-4 h-4" />
          <span class="hidden sm:inline ml-1">New</span>
        </button>

        <!-- Import -->
        <button
          class="btn btn-sm tooltip tooltip-bottom"
          data-tip="Import Games"
          :class="{ 'btn-outline': importModalOpen }"
          @click="importModalOpen = true"
        >
          <PhDownload class="w-4 h-4" />
          <span class="hidden sm:inline ml-1">Import</span>
        </button>

        <!-- Game Library Toggle -->
        <button
          class="btn btn-sm tooltip tooltip-bottom"
          data-tip="Toggle Game Library"
          :class="{ 'btn-neutral': displayGameLibrary }"
          @click="toggleGameLibraryView"
        >
          <PhBooks class="w-4 h-4" />
          <span class="hidden sm:inline ml-1">Library</span>
        </button>

        <!-- Move Tree Toggle -->
        <button
          class="btn btn-sm tooltip tooltip-bottom"
          data-tip="Toggle Move Tree"
          :class="{ 'btn-neutral': displayMoveTree }"
          @click="toggleMoveTreeView"
        >
          <PhTree class="w-4 h-4" />
          <span class="hidden sm:inline ml-1">Moves</span>
        </button>

        <!-- Engine View Toggle -->
        <button
          class="btn btn-sm tooltip tooltip-bottom"
          data-tip="Toggle Engine Analysis"
          :class="{ 'btn-neutral': displayEngineView }"
          @click="toggleEngineView"
        >
          <PhMagnifyingGlass class="w-4 h-4" />
          <span class="hidden sm:inline ml-1">Engine</span>
        </button>

        <!-- Refresh -->
        <button
          class="btn btn-sm tooltip tooltip-bottom"
          data-tip="Refresh Games"
          @click="refreshGamesClick"
        >
          <PhArrowClockwise class="w-4 h-4" />
          <span class="hidden sm:inline ml-1">Refresh</span>
        </button>
      </div>

      <!-- Right: Settings Menu -->
      <div class="navbar-end">
        <div class="dropdown dropdown-end">
          <div
            tabindex="0"
            role="button"
            class="btn btn-sm btn-circle tooltip tooltip-left"
            data-tip="More Options"
          >
            <PhGear class="w-4 h-4" />
          </div>
          <ul
            tabindex="0"
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1000] mt-3 w-48 p-2 shadow-lg border border-base-300"
          >
            <li>
              <button
                class="btn btn-sm btn-ghost justify-start"
                @click="openSettingsModal"
              >
                <PhGear class="w-4 h-4" />
                <span>Settings</span>
              </button>
            </li>
            <li>
              <button
                class="btn btn-sm btn-ghost justify-start text-error"
                @click="resetDatabaseClick"
              >
                <PhTrash class="w-4 h-4" />
                <span>Reset Database</span>
              </button>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import {
  PhArrowClockwise,
  PhBooks,
  PhDownload,
  PhGear,
  PhMagnifyingGlass,
  PhPlus,
  PhTrash,
  PhTree,
} from "@phosphor-icons/vue";
import { computed } from "vue";
import { useGlobalStore } from "../../stores";

const props = defineProps<{
  importModalOpen: boolean;
}>();

const emit = defineEmits<{
  (e: "update:importModalOpen", value: boolean): void;
  (e: "refreshGames"): void;
  (e: "resetDatabase"): void;
  (e: "newGame"): void;
}>();

const globalStore = useGlobalStore();
const uiStore = globalStore.uiStore;

const displayGameLibrary = computed(() => uiStore.getGameLibraryViewOpen);
const displayMoveTree = computed(() => uiStore.getMoveTreeViewOpen);
const displayEngineView = computed(() => uiStore.getEngineViewOpen);

const toggleGameLibraryView = () => {
  uiStore.toggleGameLibraryView();
};

const toggleMoveTreeView = () => {
  uiStore.toggleMoveTreeView();
};

const toggleEngineView = () => {
  uiStore.toggleEngineView();
};

const refreshGamesClick = () => {
  emit("refreshGames");
};

const resetDatabaseClick = () => {
  emit("resetDatabase");
};

const importModalOpen = computed({
  get: () => props.importModalOpen,
  set: (value) => emit("update:importModalOpen", value),
});

const openSettingsModal = () => {
  uiStore.updateSettingsModalOpen(true);
};

const newGameClick = () => {
  emit("newGame");
};
</script>
