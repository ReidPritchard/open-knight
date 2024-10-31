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
            contentComponent: 'MenuBarComponent',
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
                                    contentComponent: 'GameExplorerComponent',
                                    collapsible: true,
                                    collapsed: false,
                                    resizable: true,
                                    visible: true,
                                    closable: true,
                                } as IWindow,
                                {
                                    id: 'analysis-lines',
                                    contentComponent: 'AnalysisLinesComponent',
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
                                    contentComponent: 'GameBoardComponent',
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
                                    contentComponent: 'GameNotesComponent',
                                    collapsible: true,
                                    collapsed: false,
                                    resizable: true,
                                    visible: true,
                                    closable: true,
                                } as IWindow,
                                {
                                    id: 'game-headers',
                                    contentComponent: 'GameHeadersComponent',
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
                            contentComponent: 'MoveTreeComponent',
                            collapsible: true,
                            collapsed: false,
                            resizable: true,
                            visible: true,
                            closable: true,
                        } as IWindow,
                        {
                            id: 'engine-evaluation',
                            contentComponent: 'EngineEvaluationComponent',
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
            contentComponent: 'StatusBarComponent',
            fixedSize: 30, // Fixed height in pixels
            resizable: false,
            collapsible: false,
            collapsed: false,
            visible: true,
            closable: false,
        } as IWindow,
    ],
};