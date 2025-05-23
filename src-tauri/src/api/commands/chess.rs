use crate::utils::AppError;

/// Gets all legal moves for a given chess position
///
/// Parameters:
/// - `fen`: The FEN string representing the position
///
/// Returns a JSON string containing all legal moves.
#[tauri::command]
pub async fn get_legal_moves(fen: String) -> Result<String, AppError> {
    match crate::api::chess::get_legal_moves(&fen) {
        Ok(moves) => match serde_json::to_string(&moves) {
            Ok(json) => Ok(json),
            Err(e) => Err(AppError::SerializationError(e.to_string())),
        },
        Err(e) => Err(AppError::ChessError(e.to_string())),
    }
}
