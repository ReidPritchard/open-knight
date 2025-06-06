<template>

	<Teleport to="body">

		<div
			id="import-modal"
			class="modal"
			:class="{ 'modal-open': isOpen }"
		>

			<div class="modal-box">

				<div
					v-if="loading"
					class="flex flex-col justify-center items-center"
				>

					<div class="loading loading-dots loading-lg"></div>

					<p class="text-lg">Importing games...</p>

				</div>

				<div v-else>

					<form method="dialog">

						<button
							class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
							@click="onClose"
						>
							 ✕
						</button>

					</form>

					<h3 class="font-bold text-lg">Import Games</h3>

					<p class="py-4">Import games from a PGN file.</p>

					<div class="form-control">

						<label class="label cursor-pointer">

							<span class="label-text">PGN File</span>

							<input
								type="file"
								class="file-input file-input-bordered"
								@change="handleFileChange"
							/>

						</label>

						<label class="label cursor-pointer">

							<span class="label-text">PGN Text</span>

							<textarea
								class="textarea textarea-bordered"
								v-model="pgn"
							></textarea>

						</label>

						<button
							class="btn btn-primary"
							@click="importGames"
						>
							 Import
						</button>

						<button
							class="btn btn-secondary"
							@click="importDemoGames"
						>
							 Import Demo Games
						</button>

					</div>

				</div>

			</div>

			<form
				method="dialog"
				class="modal-backdrop"
			>

				<button @click="onClose">close</button>

			</form>

		</div>

	</Teleport>

</template>

<script setup lang="ts">
import { ref } from "vue";
import { useGlobalStore } from "../../stores";

const props = defineProps<{
	isOpen: boolean;
	onClose: () => void;
}>();

const globalStore = useGlobalStore();

const loading = ref(false);

const pgn = ref("");

const handleFileChange = (event: Event) => {
	const file = (event.target as HTMLInputElement).files?.[0];
	if (file) {
		const reader = new FileReader();
		reader.onload = (e) => {
			pgn.value = e.target?.result as string;
		};
		reader.readAsText(file);
	}
};

const importGames = async () => {
	loading.value = true;
	await globalStore.importPGNGames(pgn.value);
	props.onClose();
	loading.value = false;
};

const importDemoGames = async () => {
	// FIXME: Show UI for importing games
	console.warn("Import UI not implemented - loading demo games");
	// Use a pgn file from the assets folder
	try {
		const pgn = (
			await import("./../../assets/pgns/demo_multiple_games.pgn?raw")
		).default;
		await globalStore.importPGNGames(pgn);
	} catch (error) {
		console.error("Error importing demo games", error);
	}
};
</script>

