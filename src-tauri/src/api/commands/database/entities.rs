use crate::api::commands::AppState;
use crate::api::database::QueryParams;
use crate::utils::AppError;
use tauri::State;

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
    let entity =
        crate::api::database::get_entity_by_id(entity, id, None, &state.db)
            .await
            .unwrap();
    Ok(serde_json::to_string(&entity).unwrap())
}
