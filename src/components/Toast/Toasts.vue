<template>
  <Teleport to="body">
    <div class="toast toast-center">
      <div v-for="alert in alerts" :key="alert.key">
        <div
          role="alert"
          class="alert alert-vertical sm:alert-horizontal"
          :class="alert.style"
        >
          <div>
            <component :is="alert.icon" />
          </div>
          <div>
            <h3 class="font-bold">{{ alert.message }}</h3>
          </div>
          <button class="btn btn-sm" @click="uiStore.removeAlert(alert.key)">
            <PhX size="16" />
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { PhCheck, PhInfo, PhWarningDiamond, PhX } from "@phosphor-icons/vue";
import { computed } from "vue";
import { useUIStore } from "../../stores/ui";

const uiStore = useUIStore();

const alerts = computed(() =>
	uiStore.alerts.map((alert) => ({
		...alert,
		icon:
			alert.type === "success"
				? PhCheck
				: alert.type === "error"
					? PhWarningDiamond
					: PhInfo,
		style:
			alert.type === "success"
				? "alert-success"
				: alert.type === "error"
					? "alert-error"
					: "alert-info",
	})),
);
</script>
