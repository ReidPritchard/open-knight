use crate::{
    api_types::{APIGame, ExplorerGame},
    database,
};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct AppState {
    /// The currently selected game
    pub selected_game: Mutex<Option<APIGame>>,

    // Each "view" will be a separate struct
    pub explorer: Mutex<ExplorerState>,
}

/// Represents the state of the Explorer view
///
/// Used to search through a collection of games
/// can be filtered or sorted in various ways
/// and the selected game(s) can be used in other views
/// (like a game viewer, analysis tools, etc.)
#[derive(Debug, Clone, Serialize)]
pub struct ExplorerState {
    pub games: Vec<ExplorerGame>,
    // TODO: add search/filter/sort state
}

impl ExplorerState {
    pub fn new() -> Self {
        ExplorerState { games: vec![] }
    }

    pub fn clear(&mut self) {
        self.games.clear();
    }

    pub fn extend(&mut self, games: &Vec<ExplorerGame>) {
        self.games.extend(games.iter().cloned());
    }

    pub fn load_games_from_db(&mut self) {
        let games = database::game::get_all_games();

        let explorer_games = games
            .iter()
            .map(|game| ExplorerGame::from(game.clone()))
            .collect::<Vec<ExplorerGame>>();
        self.games.extend(explorer_games);
    }
}

impl Default for ExplorerState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut explorer = ExplorerState::new();
        explorer.load_games_from_db();

        AppState {
            explorer: Mutex::new(explorer),
            selected_game: Mutex::new(None),
        }
    }

    pub fn clear(&self) {
        self.explorer.lock().unwrap().clear();
        *self.selected_game.lock().unwrap() = None;
    }

    pub fn set_selected_game(&self, game: Option<APIGame>) {
        *self.selected_game.lock().unwrap() = game;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
