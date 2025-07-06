use crate::api::database::QueryParams;
use crate::models;
use crate::utils::AppError;
use crate::AppState;
use log::{error, info};
use sea_orm::sqlx::types::chrono;
use tauri::State;

/// Queries chess games from the database based on provided parameters
///
/// Returns a JSON string containing the matching games.
#[tauri::command]
pub async fn query_games(
    params: QueryParams,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    match crate::api::database::query_full_games(params, &state.db).await {
        Ok(games) => {
            info!("Successfully retrieved {} games from database", games.len());
            match serde_json::to_string(&games) {
                Ok(json) => Ok(json),
                Err(e) => {
                    error!("Error serializing games to JSON: {}", e);
                    Err(AppError::SerializationError(e.to_string()))
                }
            }
        }
        Err(e) => {
            error!("Error querying games from database: {}", e);
            Err(AppError::DatabaseError(format!(
                "Failed to query games: {}",
                e
            )))
        }
    }
}

/// Retrieves a specific chess game by its ID
///
/// Parameters:
/// - `id`: The ID of the game to retrieve
/// - `params`: Query parameters for controlling what data to load
///
/// Returns a JSON string containing the full game data.
#[tauri::command]
pub async fn get_game_by_id(
    id: i32,
    params: QueryParams,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let game = crate::api::database::get_full_game(id, params, &state.db)
        .await
        .unwrap();
    Ok(serde_json::to_string(&game).unwrap())
}

/// Deletes a game from the database
///
/// Parameters:
/// - `game_id`: The ID of the game to delete
#[tauri::command]
pub async fn delete_game(
    game_id: i32,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    // TODO: Implement a 'soft' delete (by setting a deleted flag or timestamp)
    // to allow for temporary recovery of deleted games
    models::ChessGame::delete(&state.db, game_id).await?;
    Ok(())
}

/// Updates a property on a game
///
/// Currently this is only used for inline editing of "explorer" games
/// in the game library. Meaning the properties that can be updated are limited
/// to the ones that are displayed. As of now it supports "date", "white_player_name",
/// "black_player_name" and "result". This command updates the appropriate
/// database table depending on the property being updated.
///
/// Parameters:
/// - `game_id`: The ID of the game to update
/// - `property`: The property to update ("date", "result", "white_player_name", "black_player_name")
/// - `value`: The value to set the property to
#[tauri::command]
pub async fn update_game_property(
    game_id: i32,
    property: &str,
    value: &str,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    use crate::entities::{game, player};
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    // First get the game to access related entity IDs
    let game_record = game::Entity::find_by_id(game_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to query game: {}", e))
        })?
        .ok_or_else(|| {
            AppError::DatabaseError(format!(
                "Game with ID {} not found",
                game_id
            ))
        })?;

    match property {
        "date" => {
            info!("Updating date for game {} to {}", game_id, value);

            // Create an ActiveModel for the update
            let mut game_active: game::ActiveModel = game_record.into();
            game_active.date_played = Set(Some(value.to_string()));

            // Update the record
            game_active.update(&state.db).await
                .map_err(|e| AppError::DatabaseError(format!("Failed to update game date: {}", e)))?;

            info!("Game date updated successfully!");
        }
        "result" => {
            info!("Updating result for game {} to {}", game_id, value);

            // Create an ActiveModel for the update
            let mut game_active: game::ActiveModel = game_record.into();
            game_active.result = Set(Some(value.to_string()));

            // Update the record
            game_active.update(&state.db).await
                .map_err(|e| AppError::DatabaseError(format!("Failed to update game result: {}", e)))?;

            info!("Game result updated successfully!");
        }
        "white_player_name" => {
            info!("Updating white player name for game {} to {}", game_id, value);

            // Get the white player record
            let white_player = player::Entity::find_by_id(game_record.white_player_id)
                .one(&state.db)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to query white player: {}", e)))?
                .ok_or_else(|| AppError::DatabaseError("White player not found".to_string()))?;

            // Create an ActiveModel for the update
            let mut player_active: player::ActiveModel = white_player.into();
            player_active.name = Set(value.to_string());
            player_active.updated_at = Set(Some(chrono::Utc::now()));

            // Update the record
            player_active.update(&state.db).await
                .map_err(|e| AppError::DatabaseError(format!("Failed to update white player name: {}", e)))?;

            info!("White player name updated successfully!");
        }
        "black_player_name" => {
            info!("Updating black player name for game {} to {}", game_id, value);

            // Get the black player record
            let black_player = player::Entity::find_by_id(game_record.black_player_id)
                .one(&state.db)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to query black player: {}", e)))?
                .ok_or_else(|| AppError::DatabaseError("Black player not found".to_string()))?;

            // Create an ActiveModel for the update
            let mut player_active: player::ActiveModel = black_player.into();
            player_active.name = Set(value.to_string());
            player_active.updated_at = Set(Some(chrono::Utc::now()));

            // Update the record
            player_active.update(&state.db).await
                .map_err(|e| AppError::DatabaseError(format!("Failed to update black player name: {}", e)))?;

            info!("Black player name updated successfully!");
        }
        _ => {
            return Err(AppError::DatabaseError(format!(
                "Invalid property: {}. Supported properties: date, result, white_player_name, black_player_name",
                property
            )))
        }
    }

    Ok(())
}
