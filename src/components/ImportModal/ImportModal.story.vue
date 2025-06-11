<template>

	<Story title="Import Modal">

		<div class="flex flex-col gap-4 w-full h-full">

			<ImportModal
				:is-open="true"
				@close="() => {}"
			/>

		</div>

	</Story>

</template>

<script setup lang="ts">
import ImportModal from "./ImportModal.vue";
</script>

