use crate::loader::GameResult;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState<'a> {
    // Each "view" will be a separate struct
    pub explorer: Mutex<ExplorerState<'a>>,
}

/// Represents the state of the Explorer view
///
/// Used to search through a collection of games
/// can be filtered or sorted in various ways
/// and the selected game(s) can be used in other views
/// (like a game viewer, analysis tools, etc.)
#[derive(Debug, Clone)]
pub struct ExplorerState<'a> {
    pub games: Vec<GameResult<'a>>,
    // TODO: add search/filter/sort state
}

impl<'a> ExplorerState<'a> {
    pub fn new() -> Self {
        ExplorerState { games: Vec::new() }
    }

    pub fn extend(&mut self, games: Vec<GameResult<'a>>) {
        self.games.extend(games);
    }
}

impl<'a> Default for ExplorerState<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        AppState {
            explorer: Mutex::new(ExplorerState::new()),
        }
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        Self::new()
    }
}
