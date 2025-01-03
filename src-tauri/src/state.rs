use crate::{
    database::{self, Database},
    models::{api::APIGame, game::ExplorerGame},
};
use serde::Serialize;
use std::sync::Mutex;

pub struct AppState {
    /// The database connection pool
    pub db: Database,
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

    pub fn extend(&mut self, games: &[ExplorerGame]) {
        self.games.extend(games.iter().cloned());
    }

    pub fn load_games_from_db(&mut self, db: &Database) -> Result<(), database::DatabaseError> {
        let games_with_headers = database::game::get_all_games_with_headers(db)?;
        self.games = games_with_headers
            .into_iter()
            .map(|(game, headers)| ExplorerGame::from((game, headers)))
            .collect();
        Ok(())
    }
}

impl Default for ExplorerState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Result<Self, database::DatabaseError> {
        let db = Database::new()?;
        let mut explorer = ExplorerState::new();
        explorer.load_games_from_db(&db)?;

        Ok(AppState {
            db,
            explorer: Mutex::new(explorer),
            selected_game: Mutex::new(None),
        })
    }

    pub fn clear(&self) -> Result<(), database::DatabaseError> {
        database::setup::empty_db(&self.db)?;
        self.explorer.lock().unwrap().clear();
        *self.selected_game.lock().unwrap() = None;
        Ok(())
    }

    pub fn set_selected_game(&self, game: Option<APIGame>) {
        *self.selected_game.lock().unwrap() = game;
    }

    pub fn get_db(&self) -> &Database {
        &self.db
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new().expect("Failed to create AppState")
    }
}
