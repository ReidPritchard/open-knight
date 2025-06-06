<template>

	<header class="sticky top-0 z-50 max-h-16">

		<div class="navbar bg-base-200 text-base-content px-4 select-none">

			<!-- Left: App Title -->

			<div class="navbar-start">

				<Logo
					width="48"
					height="48"
					class-name="mr-2 text-accent"
				/>

				<h1
					class="text-lg font-bold text-base-content cursor-default hidden xl:block"
				>
					 Open Knight
				</h1>

			</div>

			<!-- Center: Main Toolbar -->

			<div class="navbar-center gap-1">

				<!-- New Game -->

				<button
					class="btn btn-sm tooltip tooltip-bottom"
					data-tip="New Game"
					@click="emit('newGame')"
				>

					<PhPlus class="w-4 h-4" />

					<span class="hidden xl:inline ml-1">New</span>

				</button>

				<!-- Import -->

				<button
					class="btn btn-sm tooltip tooltip-bottom"
					data-tip="Import Games"
					:class="{ 'btn-outline': importModalOpen }"
					@click="importModalOpen = true"
				>

					<PhDownload class="w-4 h-4" />

					<span class="hidden xl:inline ml-1">Import</span>

				</button>

				<!-- Left Panel Toggle -->

				<button
					class="btn btn-sm tooltip tooltip-bottom"
					data-tip="Toggle Left Panel"
					:class="{ 'btn-neutral': displayLeftPanel }"
					@click="toggleLeftPanel"
				>

					<PhAlignLeftSimple class="w-4 h-4" />

					<span class="hidden xl:inline ml-1">Left Panel</span>

				</button>

				<!-- Right Panel Toggle -->

				<button
					class="btn btn-sm tooltip tooltip-bottom"
					data-tip="Toggle Right Panel"
					:class="{ 'btn-neutral': displayRightPanel }"
					@click="toggleRightPanel"
				>

					<PhAlignRightSimple class="w-4 h-4" />

					<span class="hidden xl:inline ml-1">Right Panel</span>

				</button>

				<!-- Refresh -->

				<button
					class="btn btn-sm tooltip tooltip-bottom"
					data-tip="Refresh Games"
					@click="refreshGamesClick"
				>

					<PhArrowClockwise class="w-4 h-4" />

					<span class="hidden xl:inline ml-1">Refresh</span>

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
	PhAlignLeftSimple,
	PhAlignRightSimple,
	PhArrowClockwise,
	PhDownload,
	PhGear,
	PhPlus,
	PhTrash,
} from "@phosphor-icons/vue";
import { computed } from "vue";
import { useGlobalStore } from "../../stores";
import Logo from "../Logo/Logo.vue";

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

const displayLeftPanel = computed(() => uiStore.getLeftPanelOpen);
const displayRightPanel = computed(() => uiStore.getRightPanelOpen);

const toggleLeftPanel = () => {
	uiStore.toggleLeftPanel();
};

const toggleRightPanel = () => {
	uiStore.toggleRightPanel();
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

