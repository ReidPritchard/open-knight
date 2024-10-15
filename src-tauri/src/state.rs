use crate::loader::GameResult;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    // Each "view" will be a separate struct
    pub explorer: Mutex<ExplorerState>,
}

/// Represents the state of the Explorer view
///
/// Used to search through a collection of games
/// can be filtered or sorted in various ways
/// and the selected game(s) can be used in other views
/// (like a game viewer, analysis tools, etc.)
#[derive(Debug, Clone)]
pub struct ExplorerState {
    pub games: Vec<GameResult>,
    // TODO: add search/filter/sort state
}

impl ExplorerState {
    pub fn new() -> Self {
        ExplorerState { games: Vec::new() }
    }

    pub fn extend(&mut self, games: &Vec<GameResult>) {
        self.games.extend(games.iter().cloned());
    }
}

impl Default for ExplorerState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            explorer: Mutex::new(ExplorerState::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
