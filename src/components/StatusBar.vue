<template>
    <div class="status-bar">
        <!-- Status bar content -->
        <p>Status Bar</p>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'StatusBarComponent',
});
</script>

<style scoped>
.status-bar {
    background-color: #f5f5f5;
    padding: 10px;
}
</style>