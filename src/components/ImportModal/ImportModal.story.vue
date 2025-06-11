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

			<div class="flex flex-col gap-4 w-full h-full">

				<ImportModal
					:is-open="isOpen"
					@close="logEvent('close', $event)"
				/>

			</div>

		</Variant>

	</Story>

</template>

<script setup lang="ts">
import { ref } from "vue";
import ImportModal from "./ImportModal.vue";
import { logEvent } from "histoire/client";

const isOpen = ref(false);
</script>

