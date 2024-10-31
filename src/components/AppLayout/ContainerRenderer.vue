<template>
    <div :style="containerStyle">
        <Panel v-if="layout.collapsible" :header="layout.title || layout.id" :collapsed="layout.collapsed" toggleable
            @toggle="onToggle">
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
        </Panel>

        <div v-else>
            <!-- Same as above but without the Panel wrapper -->
            <!-- ... -->
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, computed, ref } from 'vue';
import { IWindowContainer, validateFlexibleContainer, validateTabContainer } from '../../shared/types';
import LayoutRenderer from './LayoutRenderer.vue';
import Panel from 'primevue/panel';
import Tabs from 'primevue/tabs';
import TabPanel from 'primevue/tabpanel';

export default defineComponent({
    name: 'ContainerRenderer',
    components: {
        LayoutRenderer,
        Panel,
        Tabs,
        TabPanel,
    },
    props: {
        layout: {
            type: Object as () => IWindowContainer,
            required: true,
        },
    },
    setup(props) {
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

        return {
            orientationClass,
            containerStyle,
            activeTabIndex,
            onToggle,
        };
    },
});
</script>

<style scoped>
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