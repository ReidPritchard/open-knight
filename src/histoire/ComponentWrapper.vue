<script lang="ts" setup>
import { Story, Variant } from "histoire";

defineProps<{
	story: Story;
	variant?: Variant;
}>();
</script>

<template>

	<div
		class="p-4 border border-base-300 rounded-md min-h-[200px] min-w-[200px] w-full h-full"
	>

		<slot />

	</div>

</template>

