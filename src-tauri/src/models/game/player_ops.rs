use super::structs::ChessPlayer;
use crate::entities::*;
use crate::utils::AppError;
use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

/// Creates a default player for new games
pub async fn create_default_player(
    db: &DatabaseConnection,
    name: &str,
) -> Result<i32, AppError> {
    let player_model = player::ActiveModel {
        name: Set(name.to_string()),
        elo_rating: Set(None),
        country_code: Set(None),
        created_at: Set(Some(chrono::Utc::now())),
        updated_at: Set(Some(chrono::Utc::now())),
        ..Default::default()
    };

    let result = player::Entity::insert(player_model)
        .exec(db)
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!(
                "Failed to create default player: {}",
                e
            ))
        })?;

    Ok(result.last_insert_id)
}

/// Finds an existing player or creates a new one
pub async fn save_or_find_player<C>(
    db: &C,
    player: &ChessPlayer,
) -> Result<i32, AppError>
where
    C: ConnectionTrait,
{
    // Validate player name
    if player.name.trim().is_empty() {
        return Err(AppError::ChessError(
            "Player name cannot be empty".to_string(),
        ));
    }

    // Try to find an existing player with the same name
    if let Some(existing_player) = player::Entity::find()
        .filter(player::Column::Name.eq(&player.name))
        .one(db)
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to query player: {}", e))
        })?
    {
        // Update ELO if the new one is provided and different
        if let Some(new_elo) = player.elo {
            if existing_player.elo_rating != Some(new_elo) {
                let mut player_model: player::ActiveModel =
                    existing_player.clone().into();
                player_model.elo_rating = Set(Some(new_elo));
                player_model.updated_at = Set(Some(chrono::Utc::now()));
                player_model.update(db).await.map_err(|e| {
                    AppError::DatabaseError(format!(
                        "Failed to update player ELO: {}",
                        e
                    ))
                })?;
            }
        }
        Ok(existing_player.player_id)
    } else {
        // Create new player if not found
        let player_model = player::ActiveModel {
            name: Set(player.name.clone()),
            elo_rating: Set(player.elo),
            country_code: Set(player.country.clone()),
            created_at: Set(Some(chrono::Utc::now())),
            updated_at: Set(Some(chrono::Utc::now())),
            ..Default::default()
        };
        let result = player::Entity::insert(player_model)
            .exec(db)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!(
                    "Failed to create player: {}",
                    e
                ))
            })?;
        Ok(result.last_insert_id)
    }
}

pub fn new_player(name: &str) -> ChessPlayer {
    ChessPlayer {
        id: 0,
        name: name.to_string(),
        elo: None,
        country: None,
    }
}
