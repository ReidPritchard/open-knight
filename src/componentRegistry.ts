import type { Component } from "vue";
import AnalysisLines from "./components/AnalysisLines.vue";
import EngineEvaluation from "./components/EngineEvaluation.vue";
import GameBoard from "./components/GameBoard.vue";
import GameExplorer from "./components/GameExplorer.vue";
import GameHeaders from "./components/GameHeaders.vue";
import GameNotes from "./components/GameNotes.vue";
import MenuBar from "./components/MenuBar.vue";
import MoveTree from "./components/MoveTree.vue";
import StatusBar from "./components/StatusBar.vue";

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
