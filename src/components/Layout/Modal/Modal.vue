<template>

	<Teleport to="body">

		<div
			:id="props.id"
			class="modal"
			:class="{ 'modal-open': props.isOpen }"
			:aria-modal="props.isOpen"
			role="dialog"
			:aria-labelledby="props.title || 'modal-title'"
			:aria-describedby="props.description || 'modal-description'"
		>

			<div class="modal-box bg-base-100">

				<div
					v-if="props.loading"
					class="flex flex-col justify-center items-center"
				>

					<div class="loading loading-dots loading-lg"></div>

					<p class="text-lg">Loading...</p>

				</div>

				<div
					v-else
					class="flex flex-col gap-4"
				>

					<form method="dialog">

						<button
							class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
							@click="onClose"
						>
							 ✕
						</button>

					</form>

					<div class="modal-header">

						<slot
							v-if="$slots.header"
							name="header"
						/>

						<template v-else>

							<h3 class="font-bold text-lg"> {{ title }} </h3>

							<p class="text-sm text-gray-500"> {{ description }} </p>

						</template>

					</div>

					<slot name="content" />

					<div class="modal-footer">

						<slot
							v-if="$slots.footer"
							name="footer"
						/>

					</div>

				</div>

			</div>

		</div>

	</Teleport>

</template>

<script setup lang="ts">
const props = defineProps<{
	id: string;
	isOpen: boolean;
	title?: string;
	description?: string;
	loading?: boolean;
	onClose: () => void;
}>();
</script>

