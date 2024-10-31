<template>
    <div :style="containerStyle">
        <header class="header">
            <div v-if="layout.title">
                {{ layout.title }}
            </div>

            <div class="actions">
                <!-- Actions -->
                <Button v-if="layout.closable" icon="pi pi-close" @click="onClose" />
                <Button v-if="layout.collapsible" :icon="layout.collapsed ? 'pi pi-chevron-down' : 'pi pi-chevron-up'"
                    @click="onToggle" />
            </div>
        </header>

        <div class="content" v-if="layout.visible && !layout.collapsed">
            <div v-if="layout.display === 'flexible'" :class="['flex-container', orientationClass]">
                <template v-for="child in layout.children" :key="child.id">
                    <LayoutRenderer :layout="child" />
                </template>
            </div>

            <div v-else-if="layout.display === 'tabs'">
                <Tabs v-model:activeIndex="activeTabIndex">
                    <TabPanel v-for="(child) in layout.children" :header="child.title || child.id" :key="child.id">
                        <LayoutRenderer :layout="child" />
                    </TabPanel>
                </Tabs>
            </div>

            <div v-else-if="layout.display === 'simple'" class="simple-container">
                <div v-if="layout.top" class="top-panel">
                    <LayoutRenderer :layout="layout.top" />
                </div>
                <div class="middle-content">
                    <div v-if="layout.left" class="left-panel">
                        <LayoutRenderer :layout="layout.left" />
                    </div>
                    <div class="center-content">
                        <template v-for="child in layout.children" :key="child.id">
                            <LayoutRenderer :layout="child" />
                        </template>
                    </div>
                    <div v-if="layout.right" class="right-panel">
                        <LayoutRenderer :layout="layout.right" />
                    </div>
                </div>
                <div v-if="layout.bottom" class="bottom-panel">
                    <LayoutRenderer :layout="layout.bottom" />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { IWindowContainer, validateFlexibleContainer, validateTabContainer } from '../../shared/types';
import LayoutRenderer from './LayoutRenderer.vue';
import Tabs from 'primevue/tabs';
import TabPanel from 'primevue/tabpanel';
import Button from 'primevue/button';
const props = defineProps({
    layout: {
        type: Object as () => IWindowContainer,
        required: true,
    },
});

const orientationClass = computed(() => {
    const flexibleContainer = validateFlexibleContainer(props.layout);
    if (flexibleContainer.success) {
        return flexibleContainer.data.orientation === 'vertical'
            ? 'flex-column'
            : 'flex-row';
    }
    return '';
});

const containerStyle = computed(() => {
    const style: any = {
        display: 'flex',
        flexDirection: 'column',
    };
    if (props.layout.fixedSize) {
        style.flexBasis = `${props.layout.fixedSize}px`;
        style.flexShrink = 0;
        style.flexGrow = 0;
    } else {
        style.flexGrow = props.layout.size || 1;
    }
    return style;
});

const activeTabIndex = ref(
    props.layout.children.findIndex(
        (child) => {
            const tabContainer = validateTabContainer(child);
            return tabContainer.success && tabContainer.data.activeTabId;
        }
    )
);

const onToggle = (event: any) => {
    props.layout.collapsed = event.value;
};

const onClose = () => {
    props.layout.visible = false;
};
</script>

<style scoped>
.header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;

    padding: 0.5rem;
    width: 100%;
}

.actions {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
}

.content {
    display: flex;
    flex-direction: column;
    flex: 1;
}

.flex-container {
    display: flex;
    width: 100%;
    height: 100%;
}

.flex-row {
    flex-direction: row;
}

.flex-column {
    flex-direction: column;
}

.simple-container {
    display: flex;
    flex-direction: column;
    height: 100%;
}

.top-panel,
.bottom-panel {
    flex-shrink: 0;
}

.middle-content {
    display: flex;
    flex: 1;
}

.left-panel,
.right-panel {
    flex-shrink: 0;
}

.center-content {
    flex: 1;
    display: flex;
    flex-direction: column;
}
</style>