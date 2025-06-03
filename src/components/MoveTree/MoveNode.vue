<template>
  <template v-if="isVisible">
    <!-- Move number and move -->
    <button
      v-if="move"
      @click="$emit('select-move', nodeId)"
      class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer"
      :class="{
        'bg-primary text-primary-content': isCurrentMove,
        'hover:bg-base-300': !isCurrentMove,
      }"
    >
      <span v-if="showMoveNumber" class="font-bold"> {{ moveNumber }}. </span>
      <span v-if="!showMoveNumber && !isMainLine" class="text-base-content/60">
        ...
      </span>
      {{ move.san }}
    </button>

    <!-- Children (variations) -->
    <template v-if="shouldShowChildren">
      <!-- Main line continuation -->
      <MoveNode
        v-if="children.length === 1"
        :node="children[0]"
        :currentNodeId="currentNodeId"
        :moveTree="moveTree"
        :depth="depth + 1"
        :maxDepth="maxDepth"
        @select-move="$emit('select-move', $event)"
      />

      <!-- Multiple variations -->
      <template v-else>
        <!-- First (main) variation inline -->
        <MoveNode
          :node="children[0]"
          :currentNodeId="currentNodeId"
          :moveTree="moveTree"
          :depth="depth + 1"
          :maxDepth="maxDepth"
          @select-move="$emit('select-move', $event)"
        />

        <!-- Other variations (collapsed by default for performance) -->
        <template v-if="showVariations">
          <span
            v-for="(child, index) in children.slice(1)"
            :key="getNodeIdKey(child)"
            class="variation inline-block"
          >
            <span class="text-base-content/40">(</span>
            <MoveNode
              :node="child"
              :currentNodeId="currentNodeId"
              :moveTree="moveTree"
              :depth="depth + 1"
              :maxDepth="maxDepth"
              @select-move="$emit('select-move', $event)"
            />
            <span class="text-base-content/40">)</span>
          </span>
        </template>
        <button
          v-else-if="children.length > 1"
          @click="toggleVariations"
          class="ml-1 text-xs text-base-content/60 hover:text-base-content"
        >
          (+{{ children.length - 1 }} var{{ children.length > 2 ? "s" : "" }})
        </button>
      </template>
    </template>
  </template>
</template>

<!-- TODO: Convert to TypeScript -->
<script setup>
import { computed, ref } from "vue";

const props = defineProps({
	node: {
		type: Object,
		required: true,
	},
	currentNodeId: {
		type: Object,
		default: null,
	},
	moveTree: {
		type: Object,
		required: true,
	},
	depth: {
		type: Number,
		default: 0,
	},
	maxDepth: {
		type: Number,
		default: 10, // Limit depth for performance
	},
});

const emit = defineEmits(["select-move"]);

// State
const showVariations = ref(false);

// Computed properties
const nodeId = computed(() => getNodeId(props.node));
const move = computed(() => props.node.game_move);
const moveNumber = computed(() =>
	move.value ? Math.ceil(move.value.ply_number / 2) : 0,
);
const showMoveNumber = computed(
	() => move.value && move.value.ply_number % 2 === 1,
);

const isCurrentMove = computed(() => {
	return (
		props.currentNodeId &&
		props.currentNodeId.idx === nodeId.value.idx &&
		props.currentNodeId.version === nodeId.value.version
	);
});

const isMainLine = computed(() => {
	const parent = getParentNode();
	if (!parent) return true;
	const parentChildren = getNodeChildren(parent);
	return parentChildren[0] === props.node;
});

const children = computed(() => getNodeChildren(props.node));

// Performance optimizations
const isVisible = computed(() => {
	// Always show if we're on the path to the current move
	if (isOnCurrentPath.value) return true;

	// Limit depth for non-current paths
	return props.depth < props.maxDepth;
});

const shouldShowChildren = computed(() => {
	return children.value.length > 0 && isVisible.value;
});

const isOnCurrentPath = computed(() => {
	if (!props.currentNodeId) return false;

	// Check if this node or any ancestor is the current node
	let checkNode = props.currentNodeId;
	while (checkNode) {
		if (
			checkNode.idx === nodeId.value.idx &&
			checkNode.version === nodeId.value.version
		) {
			return true;
		}

		const node = props.moveTree.nodes[checkNode.idx]?.value;
		if (!node?.parent_id) break;
		checkNode = node.parent_id;
	}

	// Check if any descendant is the current node
	return hasCurrentNodeInSubtree(props.node);
});

// Methods
function getNodeId(node) {
	const idx = props.moveTree.nodes.findIndex((n) => n.value === node);
	const nodeWrapper = props.moveTree.nodes[idx];
	return { idx, version: nodeWrapper.version };
}

function getNodeIdKey(node) {
	const id = getNodeId(node);
	return `${id.idx}-${id.version}`;
}

function getNodeChildren(node) {
	const nodeWrapper = props.moveTree.nodes.find((n) => n.value === node);
	if (!nodeWrapper || !nodeWrapper.value) return [];

	return nodeWrapper.value.children_ids
		.map((childId) => props.moveTree.nodes[childId.idx]?.value)
		.filter(Boolean);
}

function getParentNode() {
	if (!props.node.parent_id) return null;
	return props.moveTree.nodes[props.node.parent_id.idx]?.value;
}

function hasCurrentNodeInSubtree(node) {
	if (!props.currentNodeId) return false;

	const nodeIdValue = getNodeId(node);
	if (
		props.currentNodeId.idx === nodeIdValue.idx &&
		props.currentNodeId.version === nodeIdValue.version
	) {
		return true;
	}

	const children = getNodeChildren(node);
	return children.some((child) => hasCurrentNodeInSubtree(child));
}

function toggleVariations() {
	showVariations.value = !showVariations.value;
}
</script>
