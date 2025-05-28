<template>
  <div class="flex items-center bg-base-200 border-b border-base-300 px-2 py-1">
    <!-- Board Tabs -->
    <div class="flex-1 flex items-center overflow-x-auto">
      <div role="tablist" class="tabs tabs-box bg-transparent">
        <button
          v-for="boardId in boards"
          :key="boardId"
          role="tab"
          class="select-none"
          :class="tabClasses(boardId)"
          @click="$emit('switchBoard', boardId)"
          @contextmenu.prevent="showContextMenu($event, boardId)"
          :title="getBoardTitle(boardId)"
        >
          <span class="flex-1 truncate text-sm">
            {{ getBoardDisplayName(boardId) }}
          </span>

          <!-- Unsaved changes indicator -->
          <div
            v-if="hasUnsavedChanges(boardId)"
            class="w-2 h-2 bg-warning rounded-full ml-1"
            title="Unsaved changes"
          />

          <!-- Close button -->
          <button
            v-if="boards.length > 1"
            @click.stop="$emit('closeBoard', boardId)"
            class="btn btn-xs btn-ghost btn-circle ml-1"
            :title="'Close board ' + boardId"
          >
            <PhX class="w-3 h-3" />
          </button>
        </button>
      </div>
    </div>

    <!-- New Board Button -->
    <button
      @click="createNewBoard"
      class="btn btn-sm btn-primary btn-outline ml-2"
      title="Open new board (Ctrl+T)"
    >
      <PhPlus class="w-4 h-4" />
    </button>

    <!-- Board Actions Menu -->
    <div class="dropdown dropdown-end ml-1">
      <button tabindex="0" class="btn btn-sm btn-ghost">
        <PhDotsThree class="w-4 h-4" />
      </button>
      <ul
        tabindex="0"
        class="dropdown-content menu bg-base-100 rounded-box z-10 w-52 p-2 shadow-lg border border-base-300"
      >
        <li>
          <a @click="duplicateCurrentBoard" class="flex items-center gap-2">
            <PhCopy class="w-4 h-4" />
            Duplicate Board
          </a>
        </li>
        <li>
          <a @click="closeAllOtherBoards" class="flex items-center gap-2">
            <PhX class="w-4 h-4" />
            Close Others
          </a>
        </li>
        <li v-if="boards.length > 1">
          <a @click="closeAllBoards" class="flex items-center gap-2 text-error">
            <PhX class="w-4 h-4" />
            Close All
          </a>
        </li>
        <li class="menu-title">
          <span>Recent Boards</span>
        </li>
        <li v-for="recentBoard in recentBoards" :key="recentBoard.id">
          <a @click="reopenBoard(recentBoard)" class="flex items-center gap-2">
            <PhClock class="w-4 h-4" />
            {{ recentBoard.name }}
          </a>
        </li>
      </ul>
    </div>

    <!-- Context Menu -->
    <ContextMenu
      :visible="contextMenu.visible"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :items="contextMenuItems"
      @item-click="handleContextMenuClick"
      @close="hideContextMenu"
    />
  </div>
</template>

<script setup lang="ts">
import {
	PhClock,
	PhCopy,
	PhDotsThree,
	PhPencil,
	PhPlus,
	PhX,
} from "@phosphor-icons/vue";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { ContextMenu, type MenuItem } from "../ContextMenu";

interface Props {
	boards: number[];
	activeBoard: number;
}

interface RecentBoard {
	id: number;
	name: string;
	lastUsed: Date;
}

const props = defineProps<Props>();

const emit = defineEmits<{
	switchBoard: [boardId: number];
	closeBoard: [boardId: number];
	createBoard: [];
	duplicateBoard: [boardId: number];
	renameBoard: [boardId: number, newName: string];
}>();

// Context menu state
const contextMenu = ref({
	visible: false,
	x: 0,
	y: 0,
	boardId: 0,
});

// Recent boards (would be managed by store in real implementation)
const recentBoards = ref<RecentBoard[]>([]);

// Board metadata (would come from store)
const boardMetadata = ref<
	Record<number, { name?: string; hasUnsavedChanges?: boolean }>
>({});

// Context menu items
const contextMenuItems = computed((): MenuItem[] => {
	const items: MenuItem[] = [
		{
			id: "rename",
			label: "Rename Board",
			icon: PhPencil,
		},
		{
			id: "duplicate",
			label: "Duplicate Board",
			icon: PhCopy,
		},
	];

	if (props.boards.length > 1) {
		items.push(
			{
				id: "divider",
				label: "",
				type: "divider",
			},
			{
				id: "close",
				label: "Close Board",
				icon: PhX,
				type: "destructive",
			},
		);
	}

	return items;
});

// Helper functions
const tabClasses = (boardId: number) => ({
	"tab flex items-center gap-2 max-w-48 min-w-24": true,
	"tab-active": boardId === props.activeBoard,
});

const getBoardDisplayName = (boardId: number): string => {
	const metadata = boardMetadata.value[boardId];
	if (metadata?.name) return metadata.name;
	return boardId === 0 ? "Main Board" : `Board ${boardId}`;
};

const getBoardTitle = (boardId: number): string => {
	const name = getBoardDisplayName(boardId);
	const hasChanges = hasUnsavedChanges(boardId);
	return hasChanges ? `${name} (unsaved changes)` : name;
};

const hasUnsavedChanges = (boardId: number): boolean => {
	return boardMetadata.value[boardId]?.hasUnsavedChanges ?? false;
};

// Context menu handlers
const showContextMenu = (event: MouseEvent, boardId: number) => {
	contextMenu.value = {
		visible: true,
		x: event.clientX,
		y: event.clientY,
		boardId,
	};
};

const hideContextMenu = () => {
	contextMenu.value.visible = false;
};

// Handle context menu item clicks
const handleContextMenuClick = (itemId: string) => {
	const boardId = contextMenu.value.boardId;

	switch (itemId) {
		case "rename":
			renameBoard(boardId);
			break;
		case "duplicate":
			duplicateBoard(boardId);
			break;
		case "close":
			emit("closeBoard", boardId);
			break;
	}
};

// Board actions
const createNewBoard = () => {
	emit("createBoard");
};

const duplicateCurrentBoard = () => {
	emit("duplicateBoard", props.activeBoard);
};

const duplicateBoard = (boardId: number) => {
	emit("duplicateBoard", boardId);
};

const renameBoard = (boardId: number) => {
	const currentName = getBoardDisplayName(boardId);
	const newName = prompt("Enter new board name:", currentName);

	if (newName?.trim() && newName !== currentName) {
		emit("renameBoard", boardId, newName.trim());
	}
};

const closeAllOtherBoards = () => {
	for (const boardId of props.boards) {
		if (boardId !== props.activeBoard) {
			emit("closeBoard", boardId);
		}
	}
};

const closeAllBoards = () => {
	// Close all except the first one
	for (const boardId of props.boards.slice(1)) {
		emit("closeBoard", boardId);
	}
};

const reopenBoard = (recentBoard: RecentBoard) => {
	// Implementation would depend on how boards are managed
	console.log("Reopening board:", recentBoard);
};

// Keyboard shortcuts
const handleKeyDown = (event: KeyboardEvent) => {
	// Ctrl+T: New board
	if (event.ctrlKey && event.key === "t") {
		event.preventDefault();
		createNewBoard();
	}

	// Ctrl+W: Close current board
	if (event.ctrlKey && event.key === "w" && props.boards.length > 1) {
		event.preventDefault();
		emit("closeBoard", props.activeBoard);
	}

	// Ctrl+Shift+T: Duplicate board
	if (event.ctrlKey && event.shiftKey && event.key === "T") {
		event.preventDefault();
		duplicateCurrentBoard();
	}

	// Ctrl+1-9: Switch to board by number
	if (event.ctrlKey && event.key >= "1" && event.key <= "9") {
		const index = Number.parseInt(event.key) - 1;
		if (index < props.boards.length) {
			event.preventDefault();
			emit("switchBoard", props.boards[index]);
		}
	}
};

onMounted(() => {
	document.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
	document.removeEventListener("keydown", handleKeyDown);
});
</script>

<style scoped>
/* Custom scrollbar for tab container */
.overflow-x-auto::-webkit-scrollbar {
  height: 4px;
}

.overflow-x-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-x-auto::-webkit-scrollbar-thumb {
  background: hsl(var(--bc) / 0.2);
  border-radius: 2px;
}

.overflow-x-auto::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--bc) / 0.3);
}

/* Animation for new tabs */
@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.tab-enter-active {
  animation: slideIn 0.2s ease-out;
}

/* Ensure tabs container doesn't add extra background */
.tabs.tabs-box {
  background: transparent;
  padding: 0;
}
</style>
