use std::time::Duration;

use crate::models::ChessGame;
use crate::utils::AppError;
use sea_orm::DatabaseConnection;
use tokio::time::Instant;

#[derive(serde::Serialize)]
pub struct GameSession {
    pub game: ChessGame,
    #[serde(skip)]
    pub last_saved_at: Instant,
    pub dirty: bool,
    pub move_count_since_save: u32,
}

impl GameSession {
    pub fn new(game: ChessGame) -> Self {
        Self {
            game,
            last_saved_at: Instant::now(),
            dirty: false,
            move_count_since_save: 0,
        }
    }

    pub fn should_persist(&self) -> bool {
        self.dirty
            && (self.move_count_since_save >= 10
                || self.last_saved_at.elapsed() > Duration::from_secs(30))
    }

    pub async fn persist(&mut self, _db: &DatabaseConnection) -> Result<(), AppError> {
        self.last_saved_at = Instant::now();
        self.move_count_since_save = 0;
        self.dirty = false;
        // TODO: Save to database
        Ok(())
    }

    pub async fn make_move(&mut self, move_notation: &str) -> Result<(), AppError> {
        self.game.make_move(move_notation).await?;
        self.dirty = true;
        self.move_count_since_save += 1;
        Ok(())
    }

    pub fn undo_move(&mut self) -> Result<(), AppError> {
        // TODO: Implement undo_move in ChessGame
        // For now, return an error indicating it's not implemented
        Err(AppError::ChessError(
            "Undo move not yet implemented".to_string(),
        ))
    }

    pub fn redo_move(&mut self) -> Result<(), AppError> {
        // TODO: Implement redo_move in ChessGame
        // For now, return an error indicating it's not implemented
        Err(AppError::ChessError(
            "Redo move not yet implemented".to_string(),
        ))
    }

    pub fn reset_to_position(&mut self, _move_number: usize) -> Result<(), AppError> {
        // TODO: Implement reset_to_position in ChessGame
        // For now, return an error indicating it's not implemented
        Err(AppError::ChessError(
            "Reset to position not yet implemented".to_string(),
        ))
    }

    pub fn get_move_history(&self) -> Vec<String> {
        // TODO: Implement get_move_history in ChessGame
        // For now, return an empty vector
        // This should extract moves from the move_tree
        self.game
            .move_tree
            .depth_first_move_traversal()
            .map(|m| m.san.clone())
            .collect()
    }

    pub async fn save_to_database(
        &mut self,
        db: &DatabaseConnection,
        overwrite: bool,
    ) -> Result<i32, AppError> {
        // For now, we'll always create a new game since we don't have update functionality
        // TODO: Implement proper save/update logic in ChessGame
        let _ = overwrite; // Suppress unused variable warning

        // Create a vector with just this game for the save_from_pgn method
        let pgn = self.game.to_pgn();
        match ChessGame::save_from_pgn(db, &pgn).await {
            Ok(mut games) => {
                if let Some(saved_game) = games.pop() {
                    let game_id = saved_game.id;
                    self.last_saved_at = Instant::now();
                    self.move_count_since_save = 0;
                    self.dirty = false;
                    Ok(game_id)
                } else {
                    Err(AppError::DatabaseError("No games were saved".to_string()))
                }
            }
            Err(e) => Err(AppError::DatabaseError(e.to_string())),
        }
    }
}
