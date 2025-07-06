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

    pub async fn persist(
        &mut self,
        db: &DatabaseConnection,
    ) -> Result<(), AppError> {
        self.save_to_database(db, true).await?;
        self.last_saved_at = Instant::now();
        self.move_count_since_save = 0;
        self.dirty = false;
        Ok(())
    }

    pub async fn make_move(
        &mut self,
        move_notation: &str,
    ) -> Result<(), AppError> {
        self.game.make_uci_move(move_notation).await?;
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
        let game_result = if overwrite {
            self.game.clone().save(db).await?
        } else {
            self.game.clone().update(db).await?
        };

        // Update our game with the saved version
        self.game = game_result;
        self.last_saved_at = Instant::now();
        self.move_count_since_save = 0;
        self.dirty = false;

        Ok(self.game.id)
    }
}

/// "Pass-through" methods for the game session
/// These are methods that only call methods on the game session's game
/// and don't actually mutate or use the game session.
impl GameSession {
    pub fn next_move(
        &mut self,
        variation: usize,
    ) -> Result<(), AppError> {
        self.game.move_tree.next_move(Some(variation));
        Ok(())
    }

    pub fn previous_move(&mut self) -> Result<(), AppError> {
        self.game.move_tree.previous_move();
        Ok(())
    }

    pub fn reset_to_position(
        &mut self,
        move_db_id: i32,
    ) -> Result<(), AppError> {
        self.game.move_tree.move_to_move(move_db_id);
        Ok(())
    }

    pub fn move_to_root(&mut self) -> Result<(), AppError> {
        self.game.move_tree.move_to_root();
        Ok(())
    }

    pub fn move_to_end(&mut self) -> Result<(), AppError> {
        self.game.move_tree.move_to_end();
        Ok(())
    }

    pub fn extract_positions(
        &self,
        include_variations: bool,
    ) -> Vec<crate::models::ChessPosition> {
        self.game.move_tree.extract_positions(include_variations)
    }
}
