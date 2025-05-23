<template>
  <div
    :class="[
      'relative overflow-hidden',
      direction === 'horizontal' ? 'w-full h-6' : 'h-full w-6',
      'border border-base-300 rounded-sm',
    ]"
  >
    <!-- White advantage bar (fills from left/bottom) -->
    <div
      :class="[
        'absolute bg-primary',
        direction === 'horizontal' ? 'h-full left-1/2' : 'w-full bottom-1/2',
      ]"
      :style="whiteBarStyle"
    ></div>

    <!-- Black advantage bar (fills from right/top) -->
    <div
      :class="[
        'absolute bg-secondary',
        direction === 'horizontal' ? 'h-full right-1/2' : 'w-full top-1/2',
      ]"
      :style="blackBarStyle"
    ></div>

    <!-- Center line -->
    <div
      :class="[
        'absolute bg-base-300',
        direction === 'horizontal'
          ? 'h-full w-px left-1/2 -translate-x-1/2'
          : 'w-full h-px top-1/2 -translate-y-1/2',
      ]"
    ></div>

    <!-- Evaluation text -->
    <div
      :class="[
        'absolute text-xs font-bold text-base-content flex items-center justify-center',
        direction === 'horizontal'
          ? 'h-full px-1 left-1/2 -translate-x-1/2'
          : 'w-full py-1 top-1/2 -translate-y-1/2',
      ]"
    >
      {{ barText }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { Score } from "../../shared/types";

const props = defineProps<{
	/**
	 * The evaluation of the position
	 */
	evaluation: Score;

	/**
	 * The orientation of the evaluation
	 */
	orientation: "white" | "black";

	/**
	 * The direction of the bar
	 */
	direction: "vertical" | "horizontal";
}>();

const evalValue = computed(() => props.evaluation.value);
const evalType = computed(() => props.evaluation.type);

// Format the evaluation text
const barText = computed(() => {
	if (evalType.value === "mate") {
		return `#${evalValue.value}`;
	}
	return `${(evalValue.value / 100).toFixed(1)}`;
});

// Convert evaluation to a percentage (0-100)
const evaluationPercentage = computed(() => {
	// Handle mate scores
	if (evalType.value === "mate") {
		const mateValue = evalValue.value;

		// Different handling based on whether it's mate for white or black
		const sign = mateValue > 0 ? 1 : -1;

		// Calculate a percentage that approaches 100% as mate gets closer
		// Mate in 1 should be very close to 100%
		const basePercentage = 90 + 10 / Math.min(Math.abs(mateValue), 10);

		// Adjust for which side has the mate
		return sign > 0 ? basePercentage : 100 - basePercentage;
	}

	// For regular evaluations, use a non-linear scaling function
	// This provides more resolution near 0 and compresses extreme values
	const value = evalValue.value / 100; // Convert centipawns to pawns
	const scalingFactor = 0.8; // Controls how quickly values saturate

	// Sigmoid-like function that maps to percentage
	// 50 + 50 * sigmoid gives us a range from 0-100 with 50 at 0
	let percentage = 50 + 50 * (2 / (1 + Math.exp(-scalingFactor * value)) - 1);

	// Ensure percentage is within bounds
	percentage = Math.max(0, Math.min(100, percentage));

	return percentage;
});

// Account for orientation (flip if black perspective)
const adjustedPercentage = computed(() => {
	return props.orientation === "white"
		? evaluationPercentage.value
		: 100 - evaluationPercentage.value;
});

// Calculate white advantage in percentage (0-50)
const whiteAdvantage = computed(() => {
	return Math.max(0, adjustedPercentage.value - 50);
});

// Calculate black advantage in percentage (0-50)
const blackAdvantage = computed(() => {
	return Math.max(0, 50 - adjustedPercentage.value);
});

// Generate style for white advantage bar
const whiteBarStyle = computed(() => {
	const size = `${whiteAdvantage.value * 2}%`; // Multiply by 2 as we're using half the bar

	if (props.direction === "horizontal") {
		return { width: size };
	}
	return { height: size };
});

// Generate style for black advantage bar
const blackBarStyle = computed(() => {
	const size = `${blackAdvantage.value * 2}%`; // Multiply by 2 as we're using half the bar

	if (props.direction === "horizontal") {
		return { width: size };
	}
	return { height: size };
});
</script>
