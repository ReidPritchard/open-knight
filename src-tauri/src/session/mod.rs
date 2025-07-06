use std::collections::HashMap;

use crate::models::ChessGame;
use crate::utils::AppError;
use sea_orm::DatabaseConnection;

use game_session::GameSession;

pub mod game_session;

pub struct GameSessionManager {
    /// Map of board id to game session
    active_games: HashMap<i32, GameSession>,
}

impl Default for GameSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GameSessionManager {
    pub fn new() -> Self {
        Self {
            active_games: HashMap::new(),
        }
    }

    pub fn add_game(
        &mut self,
        game: ChessGame,
        board_id: i32,
    ) {
        self.active_games.insert(board_id, GameSession::new(game));
    }

    pub fn get_session(
        &self,
        id: i32,
    ) -> Option<&GameSession> {
        self.active_games.get(&id)
    }

    pub fn get_session_mut(
        &mut self,
        id: i32,
    ) -> Option<&mut GameSession> {
        self.active_games.get_mut(&id)
    }

    pub fn get_all_sessions(&self) -> HashMap<i32, &GameSession> {
        self.active_games.iter().map(|(k, v)| (*k, v)).collect()
    }

    pub fn close_session(
        &mut self,
        id: i32,
    ) {
        self.active_games.remove(&id);
    }

    pub fn close_all_sessions(&mut self) {
        self.active_games.clear();
    }
}

/// Game session actions
impl GameSessionManager {
    pub async fn make_move(
        &mut self,
        board_id: i32,
        move_notation: &str,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.make_move(move_notation).await
    }

    pub fn undo_move(
        &mut self,
        board_id: i32,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.undo_move()
    }

    pub fn redo_move(
        &mut self,
        board_id: i32,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.redo_move()
    }

    pub fn next_move(
        &mut self,
        board_id: i32,
        variation: usize,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.next_move(variation)
    }

    pub fn previous_move(
        &mut self,
        board_id: i32,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.previous_move()
    }

    pub fn reset_to_position(
        &mut self,
        board_id: i32,
        move_db_id: i32,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.reset_to_position(move_db_id)
    }

    pub fn navigate_to_start(
        &mut self,
        board_id: i32,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.move_to_root()
    }

    pub fn navigate_to_end(
        &mut self,
        board_id: i32,
    ) -> Result<(), AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.move_to_end()
    }

    pub async fn save_session(
        &mut self,
        board_id: i32,
        db: &DatabaseConnection,
        overwrite: bool,
    ) -> Result<i32, AppError> {
        let session =
            self.get_session_mut(board_id)
                .ok_or(AppError::SessionError(
                    "Game session not found".to_string(),
                ))?;
        session.save_to_database(db, overwrite).await
    }

    pub async fn save_all_sessions(
        &mut self,
        db: &DatabaseConnection,
        overwrite: bool,
    ) -> Result<Vec<i32>, AppError> {
        let mut saved_ids = Vec::new();
        for session in self.active_games.values_mut() {
            let game_id = session.save_to_database(db, overwrite).await?;
            saved_ids.push(game_id);
        }
        Ok(saved_ids)
    }
}
