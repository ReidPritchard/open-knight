use crate::{
    chess::EditableGame,
    database::{self, Database},
    models::{api::APIGame, game::ExplorerGame},
};
use serde::Serialize;
use std::{collections::HashMap, sync::Mutex};

pub struct AppState {
    /// The database connection pool
    pub db: Database,
    /// The currently selected game
    /// TODO: Remove this in favor of the "open games" map
    pub selected_game: Mutex<Option<APIGame>>,
    // Each "view" will be a separate struct
    pub explorer: Mutex<ExplorerState>,
    /// The currently open games
    /// These games are actively being modified
    pub open_games: Mutex<HashMap<String, EditableGame>>,
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
            open_games: Mutex::new(HashMap::new()),
        })
    }

    pub fn clear(&self) -> Result<(), database::DatabaseError> {
        database::setup::empty_db(&self.db)?;
        self.explorer.lock().unwrap().clear();
        *self.selected_game.lock().unwrap() = None;
        *self.open_games.lock().unwrap() = HashMap::new();
        Ok(())
    }

    pub fn set_selected_game(&self, game: Option<APIGame>) {
        *self.selected_game.lock().unwrap() = game;
    }

    pub fn set_open_game(&self, game: EditableGame) {
        *self.open_games.lock().unwrap().insert(game.game.id, game);
    }

    pub fn get_open_game(&self, id: i32) -> Option<EditableGame> {
        self.open_games
            .lock()
            .unwrap()
            .get(&id.to_string())
            .cloned()
    }

    pub fn close_game(&self, id: i32) {
        // FIXME: Save the game to the database!!
        self.open_games.lock().unwrap().remove(&id.to_string());
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
