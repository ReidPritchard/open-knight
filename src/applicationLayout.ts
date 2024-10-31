import { IFlexibleContainer, WindowDisplayMode, LayoutDirection, IWindow, ITabContainer } from "./shared/types";


/**
 * Application layout
 * 
 * Menu bar (fixed, spans full width)
 * 
 * Left sidebar (collapsible, spans full height)
 *    (vertical collapsible child blocks/sections)
 *     Game Explorer
 *     Analysis Lines
 * 
 * Center pane (tabs, spans all available width and height, can be split to create sub-panes)
 *   Game Board(s)
 * 
 * Right sidebar (collapsible, spans full height)
 *   (vertical collapsible child blocks/sections)
 *   Game Notes
 *   Game Headers
 * 
 * Bottom bar (collapsible, spans full width)
 *   (horizontal, either tabs or collapsible child blocks/sections)
 *   Move Tree
 *   Engine Evaluation
 * 
 * Status bar (fixed, spans full width)
 * 
 */
export const applicationLayout: IFlexibleContainer = {
    id: 'app-root',
    display: WindowDisplayMode.Flexible,
    orientation: LayoutDirection.Vertical,
    collapsible: false,
    collapsed: false,
    resizable: false,
    visible: true,
    closable: false,
    children: [
        // Menu Bar
        {
            id: 'menu-bar',
            title: 'Menu Bar',
            contentComponent: 'MenuBar',
            fixedSize: 50, // Fixed height in pixels
            resizable: false,
            collapsible: false,
            collapsed: false,
            visible: true,
            closable: false,
        } as IWindow,
        // Main Area
        {
            id: 'main-area',
            display: WindowDisplayMode.Flexible,
            orientation: LayoutDirection.Vertical,
            collapsible: false,
            collapsed: false,
            resizable: true,
            visible: true,
            closable: false,
            children: [
                // Content Area
                {
                    id: 'content-area',
                    display: WindowDisplayMode.Flexible,
                    orientation: LayoutDirection.Horizontal,
                    collapsible: false,
                    collapsed: false,
                    resizable: true,
                    visible: true,
                    closable: false,
                    children: [
                        // Left Sidebar
                        {
                            id: 'left-sidebar',
                            display: WindowDisplayMode.Flexible,
                            orientation: LayoutDirection.Vertical,
                            collapsible: true,
                            collapsed: false,
                            resizable: true,
                            visible: true,
                            closable: true,
                            size: 1,
                            children: [
                                {
                                    id: 'game-explorer',
                                    title: 'Game Explorer',
                                    contentComponent: 'GameExplorer',
                                    collapsible: true,
                                    collapsed: false,
                                    resizable: true,
                                    visible: true,
                                    closable: true,
                                } as IWindow,
                                {
                                    id: 'analysis-lines',
                                    title: 'Analysis Lines',
                                    contentComponent: 'AnalysisLines',
                                    collapsible: true,
                                    collapsed: false,
                                    resizable: true,
                                    visible: true,
                                    closable: true,
                                } as IWindow,
                            ],
                        } as IFlexibleContainer,
                        // Center Pane
                        {
                            id: 'center-pane',
                            display: WindowDisplayMode.Tabs,
                            collapsible: false,
                            collapsed: false,
                            resizable: true,
                            visible: true,
                            closable: false,
                            size: 2,
                            activeTabId: 'game-board-1',
                            children: [
                                {
                                    id: 'game-board-1',
                                    title: 'Game Board 1',
                                    contentComponent: 'GameBoard',
                                    collapsible: false,
                                    collapsed: false,
                                    resizable: true,
                                    visible: true,
                                    closable: true,
                                } as IWindow,
                                // Additional game boards can be added here
                            ],
                        } as ITabContainer,
                        // Right Sidebar
                        {
                            id: 'right-sidebar',
                            display: WindowDisplayMode.Flexible,
                            orientation: LayoutDirection.Vertical,
                            collapsible: true,
                            collapsed: false,
                            resizable: true,
                            visible: true,
                            closable: true,
                            size: 1,
                            children: [
                                {
                                    id: 'game-notes',
                                    title: 'Game Notes',
                                    contentComponent: 'GameNotes',
                                    collapsible: true,
                                    collapsed: false,
                                    resizable: true,
                                    visible: true,
                                    closable: true,
                                } as IWindow,
                                {
                                    id: 'game-headers',
                                    title: 'Game Headers',
                                    contentComponent: 'GameHeaders',
                                    collapsible: true,
                                    collapsed: false,
                                    resizable: true,
                                    visible: true,
                                    closable: true,
                                } as IWindow,
                            ],
                        } as IFlexibleContainer,
                    ],
                } as IFlexibleContainer,
                // Bottom Bar
                {
                    id: 'bottom-bar',
                    title: '',
                    display: WindowDisplayMode.Flexible,
                    orientation: LayoutDirection.Horizontal,
                    collapsible: true,
                    collapsed: false,
                    resizable: true,
                    visible: true,
                    closable: true,
                    fixedSize: 200, // Fixed height in pixels
                    children: [
                        {
                            id: 'move-tree',
                            title: 'Move Tree',
                            contentComponent: 'MoveTree',
                            collapsible: true,
                            collapsed: false,
                            resizable: true,
                            visible: true,
                            closable: true,
                        } as IWindow,
                        {
                            id: 'engine-evaluation',
                            title: 'Engine Evaluation',
                            contentComponent: 'EngineEvaluation',
                            collapsible: true,
                            collapsed: false,
                            resizable: true,
                            visible: true,
                            closable: true,
                        } as IWindow,
                    ],
                } as IFlexibleContainer,
            ],
        } as IFlexibleContainer,
        // Status Bar
        {
            id: 'status-bar',
            contentComponent: 'StatusBar',
            fixedSize: 30, // Fixed height in pixels
            resizable: false,
            collapsible: false,
            collapsed: false,
            visible: true,
            closable: false,
        } as IWindow,
    ],
};