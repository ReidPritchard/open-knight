import { createGlobalState } from '@vueuse/core'
import { ref, computed } from 'vue'
import { applicationLayout } from '../applicationLayout';
import { ILayout, IWindowContainer, validateWindowContainer, validateWindow, IWindow, IGame } from './types';

// Helper to get the visible windows from the layout and its children
function getVisibleWindowsHelper(layout: ILayout): IWindow[] {
    const visibleWindows: IWindow[] = [];

    // If it's a container, check that it's visible and not collapsed
    // and then recursively check its children
    if (validateWindowContainer(layout).success) {
        const container = layout as IWindowContainer;
        if (container.visible && !container.collapsed) {
            container.children.forEach((child) => {
                const visibleChildWindows = getVisibleWindowsHelper(child);
                visibleWindows.push(...visibleChildWindows);
            });
        }
    }

    // If it's a window, check that it's visible and not collapsed
    if (validateWindow(layout).success) {
        const window = layout as IWindow;
        if (window.visible && !window.collapsed) {
            visibleWindows.push(window);
        }
    }

    return visibleWindows;
}

export const useGlobalState = createGlobalState(() => {
    // Global state
    const layout = ref<ILayout>(applicationLayout);
    const selectedGame = ref<IGame | null>(null);

    // Getters
    const getLayout = computed(() => layout.value);
    const getVisibleWindows = computed(() => getVisibleWindowsHelper(layout.value));
    const getSelectedGame = computed(() => selectedGame.value);

    // Actions
    const setLayout = (newLayout: ILayout) => {
        layout.value = newLayout;
    };

    const setSelectedGame = (newGame: IGame) => {
        selectedGame.value = newGame;
    };

    return { layout, getLayout, getVisibleWindows, getSelectedGame, setLayout, setSelectedGame };
});
