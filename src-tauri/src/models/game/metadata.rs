use super::structs::{ChessOpening, ChessTournament};
use crate::entities::*;
use crate::utils::AppError;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::ConnectionTrait;

/// Saves a tournament to the database and returns its ID
pub async fn save_tournament<C>(db: &C, tournament: &ChessTournament) -> Result<i32, AppError>
where
    C: ConnectionTrait,
{
    let tournament_model = tournament::ActiveModel {
        name: Set(tournament.name.clone()),
        r#type: Set(tournament.tournament_type.clone()),
        time_control: Set(tournament.time_control.clone()),
        start_date: Set(tournament.start_date.clone()),
        end_date: Set(tournament.end_date.clone()),
        location: Set(tournament.location.clone()),
        ..Default::default()
    };

    let result = tournament::Entity::insert(tournament_model)
        .exec(db)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to save tournament: {}", e)))?;

    Ok(result.last_insert_id)
}

/// Saves an opening to the database and returns its ID
pub async fn save_opening<C>(db: &C, opening: &ChessOpening) -> Result<i32, AppError>
where
    C: ConnectionTrait,
{
    let opening_model = opening::ActiveModel {
        eco_code: Set(opening.eco.clone()),
        name: Set(opening
            .name
            .clone()
            .unwrap_or_else(|| "Unknown".to_string())),
        variation: Set(opening.variation.clone()),
        ..Default::default()
    };

    let result = opening::Entity::insert(opening_model)
        .exec(db)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to save opening: {}", e)))?;

    Ok(result.last_insert_id)
}

/// Loads a tournament from the database by ID
pub async fn load_tournament(
    db: &DatabaseConnection,
    tournament_id: i32,
) -> Result<ChessTournament, AppError> {
    let tournament = tournament::Entity::find_by_id(tournament_id)
        .one(db)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to query tournament: {}", e)))?
        .ok_or_else(|| {
            AppError::DatabaseError(format!("Tournament with ID {} not found", tournament_id))
        })?;

    Ok(ChessTournament {
        id: tournament.tournament_id,
        name: tournament.name,
        tournament_type: tournament.r#type,
        time_control: tournament.time_control,
        start_date: tournament.start_date,
        end_date: tournament.end_date,
        location: tournament.location,
    })
}

/// Loads an opening from the database by ID
pub async fn load_opening(
    db: &DatabaseConnection,
    opening_id: i32,
) -> Result<ChessOpening, AppError> {
    let opening = opening::Entity::find_by_id(opening_id)
        .one(db)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to query opening: {}", e)))?
        .ok_or_else(|| {
            AppError::DatabaseError(format!("Opening with ID {} not found", opening_id))
        })?;

    Ok(ChessOpening {
        id: opening.opening_id,
        eco: opening.eco_code.map(|s| s.to_string()),
        name: Some(opening.name),
        variation: opening.variation.map(|s| s.to_string()),
    })
}
