import MenuBar from './components/MenuBar.vue';
import StatusBar from './components/StatusBar.vue';
import GameExplorer from './components/GameExplorer.vue';
import AnalysisLines from './components/AnalysisLines.vue';
import GameBoard from './components/GameBoard.vue';
import GameNotes from './components/GameNotes.vue';
import GameHeaders from './components/GameHeaders.vue';
import MoveTree from './components/MoveTree.vue';
import EngineEvaluation from './components/EngineEvaluation.vue';
import { Component } from 'vue';

const componentRegistry: Record<string, Component> = {
    MenuBar,
    StatusBar,
    GameExplorer,
    AnalysisLines,
    GameBoard,
    GameNotes,
    GameHeaders,
    MoveTree,
    EngineEvaluation,
};

export default componentRegistry;
