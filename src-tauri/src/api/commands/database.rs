use super::AppState;
use crate::api::database::QueryParams;
use crate::db::reset_database;
use crate::models;
use crate::utils::AppError;
use tauri::State;

/// Resets the database to its initial empty state
///
/// This command removes all data from the database.
/// Useful for testing and development.
#[tauri::command]
pub async fn empty_db(state: State<'_, AppState>) -> Result<(), AppError> {
    reset_database(&state.db).await?;
    Ok(())
}

/// Imports chess games from PGN format
///
/// Parses the provided PGN string and saves the games to the database.
/// Returns the number of successfully parsed games.
#[tauri::command]
pub async fn import_pgn_games(pgn: &str, state: State<'_, AppState>) -> Result<String, AppError> {
    let games = models::ChessGame::save_from_pgn(&state.db, &pgn).await?;
    Ok(format!("Successfully parsed {} games", games.len()))
}

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
            println!("Successfully retrieved {} games from database", games.len());
            match serde_json::to_string(&games) {
                Ok(json) => Ok(json),
                Err(e) => {
                    eprintln!("Error serializing games to JSON: {}", e);
                    Err(AppError::SerializationError(e.to_string()))
                }
            }
        }
        Err(e) => {
            eprintln!("Error querying games from database: {}", e);
            Err(AppError::DatabaseError(format!(
                "Failed to query games: {}",
                e
            )))
        }
    }
}

/// Queries entities of a specific type from the database
///
/// Parameters:
/// - `entity`: The type of entity to query (e.g., "player", "event")
/// - `params`: Query parameters for filtering
///
/// Returns a JSON string containing the matching entities.
#[tauri::command]
pub async fn query_entities(
    entity: &str,
    params: QueryParams,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let games = crate::api::database::query_entities(entity, params, &state.db)
        .await
        .unwrap();

    Ok(serde_json::to_string(&games).unwrap())
}

/// Retrieves a specific entity by its ID
///
/// Parameters:
/// - `entity`: The type of entity to retrieve (e.g., "player", "event")
/// - `id`: The ID of the entity
///
/// Returns a JSON string containing the entity data.
#[tauri::command]
pub async fn get_entity_by_id(
    entity: &str,
    id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let entity = crate::api::database::get_entity_by_id(entity, id, None, &state.db)
        .await
        .unwrap();
    Ok(serde_json::to_string(&entity).unwrap())
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
pub async fn delete_game(game_id: i32, state: State<'_, AppState>) -> Result<(), AppError> {
    // TODO: Implement a 'soft' delete (by setting a deleted flag or timestamp)
    // to allow for temporary recovery of deleted games
    models::ChessGame::delete(&state.db, game_id).await?;
    Ok(())
}
