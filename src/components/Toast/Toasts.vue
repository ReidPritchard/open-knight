<template>

	<Teleport to="body">

		<div class="toast toast-center">

			<div
				v-for="alert in alerts"
				:key="alert.key"
			>

				<div
					role="alert"
					class="alert alert-vertical sm:alert-horizontal"
					:class="alert.style"
				>

					<div>

						<component :is="alert.icon" />

					</div>

					<div class="flex flex-col gap-1 text-left">

						<h3 class="text-sm font-bold">{{ alert.title }}</h3>

						<p class="text-xs">{{ alert.message }}</p>

					</div>

					<button
						class="btn btn-sm"
						@click="uiStore.removeAlert(alert.key)"
					>

						<PhX size="16" />

					</button>

				</div>

			</div>

		</div>

	</Teleport>

</template>

<script setup lang="ts">
import { PhCheck, PhInfo, PhWarningDiamond, PhX } from "@phosphor-icons/vue";
import { type Component, computed } from "vue";
import type { AlertToast } from "../../shared/types";
import { useUIStore } from "../../stores/ui";

const uiStore = useUIStore();

const alertIcons: Record<AlertToast["type"], Component> = {
	success: PhCheck,
	error: PhWarningDiamond,
	info: PhInfo,
	warning: PhWarningDiamond,
};

const alertTitles: Record<AlertToast["type"], string> = {
	success: "Success",
	error: "Error",
	info: "Info",
	warning: "Warning",
};

const alerts = computed(() =>
	uiStore.alerts.map((alert) => ({
		...alert,
		icon: alertIcons[alert.type],
		title: alert.title ?? alertTitles[alert.type],
		style: `alert-${alert.type} text-${alert.type}-content`,
	})),
);
</script>

