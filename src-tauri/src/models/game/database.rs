use super::metadata;
use super::player_ops;
use super::structs::ChessGame;
use crate::entities::*;
use crate::utils::AppError;
use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ConnectionTrait, DatabaseTransaction, TransactionTrait};

/// Common structure for game metadata operations
pub struct GameMetadata {
    pub white_player_id: i32,
    pub black_player_id: i32,
    pub tournament_id: Option<i32>,
    pub opening_id: Option<i32>,
}

/// Saves or updates game metadata (players, tournament, opening) and returns the metadata IDs
pub async fn save_game_metadata<C>(db: &C, game: &ChessGame) -> Result<GameMetadata, AppError>
where
    C: ConnectionTrait,
{
    // Save or find players
    let white_player_id = player_ops::save_or_find_player(db, &game.white_player).await?;
    let black_player_id = player_ops::save_or_find_player(db, &game.black_player).await?;

    // Save tournament if exists
    let tournament_id = if let Some(t) = &game.tournament {
        Some(metadata::save_tournament(db, t).await?)
    } else {
        None
    };

    // Save opening if exists
    let opening_id = if let Some(o) = &game.opening {
        Some(metadata::save_opening(db, o).await?)
    } else {
        None
    };

    Ok(GameMetadata {
        white_player_id,
        black_player_id,
        tournament_id,
        opening_id,
    })
}

/// Creates a game database model from ChessGame and metadata
pub fn create_game_model(
    game: &ChessGame,
    metadata: &GameMetadata,
    game_id: Option<i32>,
) -> game::ActiveModel {
    let game_date = if game.date == "????-??-??" {
        None
    } else {
        Some(game.date.clone())
    };

    let mut model = game::ActiveModel {
        white_player_id: Set(metadata.white_player_id),
        black_player_id: Set(metadata.black_player_id),
        tournament_id: Set(metadata.tournament_id),
        opening_id: Set(metadata.opening_id),
        result: Set(Some(game.result.clone())),
        round_number: Set(game.round),
        date_played: Set(game_date),
        fen: Set(game.fen.clone()),
        pgn: Set(game.pgn.clone().unwrap_or_default()),
        ..Default::default()
    };

    if let Some(id) = game_id {
        model.game_id = Set(id);
        // Don't update created_at for existing games
        model.created_at = sea_orm::ActiveValue::NotSet;
    } else {
        model.created_at = Set(Some(chrono::Utc::now()));
    }

    model
}

/// Helper function to begin a transaction
pub async fn begin_transaction(db: &DatabaseConnection) -> Result<DatabaseTransaction, AppError> {
    db.begin()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to start transaction: {}", e)))
}

/// Helper function to commit a transaction
pub async fn commit_transaction(txn: DatabaseTransaction) -> Result<(), AppError> {
    txn.commit()
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to commit transaction: {}", e)))
}

/// Helper function to rollback a transaction (ignores errors)
pub async fn rollback_transaction(txn: DatabaseTransaction) {
    let _ = txn.rollback().await;
}

/// Saves moves for a game, deleting existing moves first if updating
pub async fn save_game_moves<C>(db: &C, game: &ChessGame, is_update: bool) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    // Delete existing moves if this is an update
    if is_update {
        r#move::Entity::delete_many()
            .filter(r#move::Column::GameId.eq(game.id))
            .exec(db)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to delete existing moves: {}", e))
            })?;
    }

    // Update game_id for all moves and save them using tree structure
    let mut updated_tree = game.move_tree.clone();
    updated_tree.game_id = game.id;

    // Update game_id for all moves in the tree
    for (_, node) in updated_tree.nodes.iter_mut() {
        if let Some(ref mut chess_move) = node.game_move {
            chess_move.game_id = game.id;
        }
    }

    // Save moves using the tree structure to preserve variations
    updated_tree
        .save_moves_to_db(db)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to save moves: {}", e)))?;

    Ok(())
}
