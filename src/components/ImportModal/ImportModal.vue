<template>

	<Modal
		id="import-modal"
		title="Import Games"
		description="Import games from a PGN file."
		:is-open="isOpen"
		:on-close="onClose"
	>

		<template #content>

			<fieldset
				class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4"
			>

				<legend class="fieldset-legend">PGN Import</legend>

				<div class="flex flex-row gap-4 items-center justify-evenly">

					<label class="label cursor-pointer bg-base-200">

						<input
							type="radio"
							name="pgn-input-type"
							class="radio radio-primary"
							v-model="inputType"
							value="file"
						/>

						<span class="label">Upload File</span>

					</label>

					<div class="divider divider-horizontal">OR</div>

					<label class="label cursor-pointer bg-base-200">

						<input
							type="radio"
							name="pgn-input-type"
							class="radio radio-primary"
							v-model="inputType"
							value="raw"
						/>

						<span class="label">Raw Text</span>

					</label>

				</div>

				<template v-if="inputType === 'file'">

					<label
						class="label"
						for="pgn-file"
					>
						 PGN File
					</label>

					<input
						type="file"
						id="pgn-file"
						required
						class="file-input file-input-bordered validator"
						@change="handleFileChange"
					/>

					<p class="validator-hint">Must be a valid PGN file</p>

				</template>

				<template v-if="inputType === 'raw'">

					<label
						class="label"
						for="pgn-text"
					>
						 PGN Text
					</label>

					<textarea
						ref="pgnTextarea"
						v-model="pgn"
						id="pgn-text"
						required
						class="textarea h-24 w-full validator"
						placeholder="PGN Text"
					></textarea>

					<p class="validator-hint">Valid PGN text required</p>

				</template>

				<div class="join mt-4">

					<button
						class="join-item btn btn-error"
						@click="onClose"
					>
						 Cancel
					</button>

					<button
						class="join-item btn btn-primary grow"
						:disabled="!pgnValid || loading"
						@click="importGames"
					>
						 Import Games
					</button>

				</div>

			</fieldset>

		</template>

		<template #footer>

			<div class="flex flex-col gap-4">

				<button
					class="btn btn-neutral"
					@click="importDemoGames"
				>
					 Import Demo Games
				</button>

			</div>

		</template>

	</Modal>

</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useGlobalStore } from "../../stores";
import Modal from "../Layout/Modal/Modal.vue";
import { validatePGNFormat } from "../../services/ImportExportService";

const props = defineProps<{
	isOpen: boolean;
	onClose: () => void;
}>();

const globalStore = useGlobalStore();

const loading = ref(false);

const inputType = ref<"file" | "raw">("file");
const pgn = ref<string>("");
const pgnValid = computed(() => {
	return validatePGNFormat(pgn.value).isValid;
});

const pgnTextarea = ref<HTMLTextAreaElement | null>(null);

watch(
	pgnValid,
	() => {
		if (!pgnValid.value) {
			// set the input to invalid
			pgnTextarea.value?.setCustomValidity("Text is not a valid PGN");
			pgnTextarea.value?.reportValidity();
		} else {
			// set the input to valid
			pgnTextarea.value?.setCustomValidity("");
			pgnTextarea.value?.reportValidity();
		}
	},
	{
		immediate: true,
	},
);

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
	// Use a pgn file from the assets folder
	// - "demo_multiple_games.pgn"
	// - "demo_single.pgn"
	// - "best-chess-games-collection-p1.pgn"
	try {
		const pgn = (
			await import("./../../assets/pgns/best-chess-games-collection-p1.pgn?raw")
		).default;
		await globalStore.importPGNGames(pgn);
	} catch (error) {
		console.error("Error importing demo games", error);
	}
};
</script>

