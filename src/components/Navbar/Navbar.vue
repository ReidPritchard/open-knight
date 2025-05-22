<template>
  <header>
    <div class="navbar bg-base-200 text-primary">
      <div class="navbar-start">
        <div class="dropdown">
          <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h16M4 18h7"
              />
            </svg>
          </div>
          <ul
            tabindex="0"
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1000 mt-3 w-52 p-2 shadow-sm"
          >
            <li>
              <button class="btn btn-ghost">
                <span class="material-symbols-outlined"> home </span>
              </button>
            </li>
            <li>
              <button class="btn btn-ghost" @click="toggleGameLibraryView">
                <span
                  class="material-symbols-outlined"
                  :class="{ 'text-primary': displayGameLibrary }"
                >
                  explore
                </span>
              </button>
            </li>
            <li>
              <button
                class="btn btn-ghost"
                @click="toggleMoveTreeView"
                :class="{ 'text-primary': displayMoveTree }"
              >
                move tree
              </button>
            </li>
            <li>
              <button class="btn btn-ghost" @click="importModalOpen = true">
                <span class="material-symbols-outlined"> import </span>
              </button>
            </li>
            <li>
              <button class="btn btn-ghost" @click="refreshGamesClick">
                <span class="material-symbols-outlined"> refresh </span>
              </button>
            </li>
            <li>
              <button class="btn btn-ghost" @click="resetDatabaseClick">
                <span class="material-symbols-outlined"> reset database </span>
              </button>
            </li>
            <li>
              <button class="btn btn-ghost" @click="openSettingsModal">
                <span class="material-symbols-outlined"> settings </span>
              </button>
            </li>
          </ul>
        </div>
      </div>
      <div class="navbar-center">
        <a class="btn btn-ghost text-xl"> Open Knight </a>
      </div>
      <div class="navbar-end">
        <button
          class="btn btn-ghost btn-circle"
          @click="toggleEngineView"
          :class="{ 'text-primary': displayEngineView }"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGlobalStore } from "../../stores";

const props = defineProps<{
  importModalOpen: boolean;
}>();

const emit = defineEmits<{
  (e: "update:importModalOpen", value: boolean): void;
  (e: "refreshGames"): void;
  (e: "resetDatabase"): void;
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
</script>
