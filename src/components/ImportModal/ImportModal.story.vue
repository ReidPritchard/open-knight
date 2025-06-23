<template>

	<Story title="Import Modal">

		<template #controls>
			 Is Open:
			<HstCheckbox
				v-model="isOpen"
				title="Boolean"
			/>

		</template>

		<Variant title="Default">

			<button
				class="btn btn-primary mb-4"
				@click="isOpen = true"
			>
				 Open Modal
			</button>

			<ImportModal
				:is-open="isOpen"
				@close="
					logEvent('close', $event);
					isOpen = false;
				"
			/>

		</Variant>

	</Story>

</template>

<script setup lang="ts">
import { ref } from "vue";
import ImportModal from "./ImportModal.vue";
import { logEvent } from "histoire/client";

const isOpen = ref(false);
</script>

