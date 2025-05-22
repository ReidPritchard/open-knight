<template>
  <header>
    <div class="navbar bg-base-200 text-base-content">
      <div class="navbar-start">
        <div class="dropdown">
          <div
            tabindex="0"
            role="button"
            class="btn btn-circle text-base-content"
          >
            <PhList />
          </div>
          <ul
            tabindex="0"
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1000 mt-3 w-52 p-2 shadow-sm gap-1 text-base-content"
          >
            <li>
              <button
                class="btn"
                :class="{
                  'btn-active': displayGameLibrary,
                }"
                @click="toggleGameLibraryView"
              >
                <span> explore </span>
              </button>
            </li>
            <li>
              <button
                class="btn"
                @click="toggleMoveTreeView"
                :class="{ 'btn-active': displayMoveTree }"
              >
                move tree
              </button>
            </li>
            <li>
              <button
                class="btn"
                @click="importModalOpen = true"
                :class="{ 'btn-active': importModalOpen }"
              >
                <span> import </span>
              </button>
            </li>
            <li>
              <button class="btn" @click="refreshGamesClick">
                <span> refresh </span>
              </button>
            </li>
            <li>
              <button class="btn" @click="resetDatabaseClick">
                <span> reset database </span>
              </button>
            </li>
            <li>
              <button class="btn" @click="openSettingsModal">
                <span> settings </span>
              </button>
            </li>
          </ul>
        </div>

        <button class="btn" @click="newGameClick">
          <PhPlus />
        </button>
      </div>
      <div class="navbar-center">
        <h1 class="text-xl font-bold text-base-content cursor-default">
          Open Knight
        </h1>
      </div>
      <div class="navbar-end">
        <button class="btn btn-circle" @click="toggleEngineView">
          <PhMagnifyingGlass />
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { PhList, PhMagnifyingGlass, PhPlus } from "@phosphor-icons/vue";
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
