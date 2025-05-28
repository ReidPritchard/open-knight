<template>
  <div class="collapse collapse-arrow border border-base-300">
    <input type="checkbox" />
    <h4 class="collapse-title text-sm font-semibold">Engine Settings</h4>
    <div class="collapse-content gap-2">
      <div
        v-for="([name, option], idx) in engineSettings"
        :key="name"
        class="flex items-center justify-between p-1"
      >
        <span class="w-40 text-xs opacity-70">{{ name }}</span>
        <template v-if="option.type === 'string'">
          <input
            :id="name"
            type="text"
            v-model="option.value"
            class="input input-sm w-full"
            @input="onInput(idx, option.value)"
          />
        </template>
        <template v-else-if="option.type === 'spin'">
          <input
            :id="name"
            type="number"
            v-model.number="option.value"
            class="input input-sm w-28"
            @input="onInput(idx, option.value)"
            :min="option.min"
            :max="option.max"
          />
        </template>
        <template v-else-if="option.type === 'check'">
          <input
            :id="name"
            type="checkbox"
            v-model="option.value"
            class="checkbox checkbox-sm"
            @change="onInput(idx, option.value)"
          />
        </template>
        <template v-else-if="option.type === 'combo'">
          <select
            :id="name"
            v-model="option.value"
            class="select select-sm w-full"
            @change="onInput(idx, option.value)"
          >
            <option v-for="v in option.var || []" :key="v" :value="v">
              {{ v }}
            </option>
          </select>
        </template>
        <template v-else-if="option.type === 'button'">
          <button
            type="button"
            class="btn btn-sm btn-primary"
            @click="onInput(idx, option.value)"
          >
            {{ name }}
          </button>
        </template>
        <template v-else>
          <input
            :id="name"
            type="text"
            v-model="option.value"
            class="input input-sm w-full"
            @input="onInput(idx, option.value)"
          />
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EngineOption } from "../../shared/types";

const props = defineProps<{
	engineSettings: [string, EngineOption][];
}>();
const emit =
	defineEmits<
		(event: "update:engineSettings", value: [string, EngineOption][]) => void
	>();

function onInput(idx: number, value: unknown) {
	// Emit the updated settings array
	const updated = props.engineSettings.map((entry, i) =>
		i === idx
			? ([entry[0], { ...entry[1], value }] as [string, EngineOption])
			: entry,
	);
	emit("update:engineSettings", updated);
}
</script>
