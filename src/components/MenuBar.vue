<template>
    <Menubar :model="items" />
</template>

<script setup lang="ts">
import Menubar from 'primevue/menubar';
import { MenuItem, MenuItemCommandEvent } from 'primevue/menuitem';
import { ref } from 'vue';
import { IWindowContainer } from '../shared/types';


const onCommand = (event: MenuItemCommandEvent) => {
    console.log(event);
};

const onViewCommand = (event: MenuItemCommandEvent) => {
    console.log(event);
};

const props = defineProps<{
    layout?: IWindowContainer;
}>();

// Check which windows are open
// TODO: improve this
const openWindows = ref(
    props.layout?.children.filter((child) => child.visible)
);

const items = ref([
    {
        label: 'File',
        icon: 'pi pi-fw pi-file',
        visible: true,
        command: onCommand,
        items: [
            {
                label: 'Load PGN',
                icon: 'pi pi-fw pi-file-import',
                command: onCommand,
            },
            {
                label: 'Export PGN',
                icon: 'pi pi-fw pi-file-export',
                command: onCommand,
            },
            {
                label: 'Quit',
                icon: 'pi pi-fw pi-power-off',
                command: onCommand,
            },
        ],
    } as MenuItem,
    {
        label: 'View',
        icon: 'pi pi-fw pi-eye',
        visible: true,
        command: onCommand,
        items: [
            {
                label: 'Board',
                icon:
                    openWindows.value?.find((child) => child.id === 'board')
                        ?.icon || 'pi pi-fw pi-board',
                command: onViewCommand,
            },
            {
                label: 'Game',
                icon:
                    openWindows.value?.find((child) => child.id === 'game')
                        ?.icon || 'pi pi-fw pi-game',
                command: onViewCommand,
            },
            {
                label: 'Analysis',
                icon:
                    openWindows.value?.find((child) => child.id === 'analysis')
                        ?.icon || 'pi pi-fw pi-analysis',
                command: onViewCommand,
            },
            {
                label: 'Settings',
                icon:
                    openWindows.value?.find((child) => child.id === 'settings')
                        ?.icon || 'pi pi-fw pi-settings',
                command: onViewCommand,
            },
        ],
    } as MenuItem,
]);
</script>

<style scoped>
.menu-bar {
    background-color: #f5f5f5;
    padding: 10px;
}
</style>