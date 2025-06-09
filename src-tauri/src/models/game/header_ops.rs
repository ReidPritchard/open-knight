use crate::entities::game_header;
use crate::models::structs::ChessHeader;
use crate::utils::AppError;
use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;

impl From<ChessHeader> for game_header::ActiveModel {
    fn from(header: ChessHeader) -> Self {
        let now = chrono::Utc::now();
        let header_id = if header.id.is_some() {
            Set(header.id.unwrap())
        } else {
            sea_orm::ActiveValue::NotSet
        };

        game_header::ActiveModel {
            header_id,
            game_id: Set(header.game_id),
            header_name: Set(header.name),
            header_value: Set(header.value),
            // TODO: These should probably be set by the database
            created_at: Set(now),
            updated_at: Set(now),
        }
    }
}

impl From<game_header::Model> for ChessHeader {
    fn from(header: game_header::Model) -> Self {
        ChessHeader {
            id: Some(header.header_id),
            game_id: header.game_id,
            name: header.header_name,
            value: header.header_value,
        }
    }
}

/// Finds a header in the database by ChessHeader struct
pub async fn find_db_header<C>(
    db: &C,
    header: &ChessHeader,
) -> Result<Option<game_header::Model>, AppError>
where
    C: ConnectionTrait,
{
    let result = game_header::Entity::find()
        .filter(game_header::Column::HeaderId.eq(header.id.unwrap_or(0)))
        .filter(game_header::Column::GameId.eq(header.game_id))
        .one(db)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find header: {}", e)))?;

    Ok(result)
}

/// Inserts or updates a header in the database
pub async fn save_header<C>(db: &C, header: &ChessHeader) -> Result<ChessHeader, AppError>
where
    C: ConnectionTrait,
{
    let local_header = header.clone();

    // Try to find an existing header with the same name and value
    let existing_row = find_db_header(db, header).await?;
    if let Some(existing_header) = existing_row {
        // Update the existing header entry
        let mut header_model: game_header::ActiveModel = existing_header.into();
        header_model.header_value = Set(local_header.value);
        header_model.updated_at = Set(chrono::Utc::now());
        let result = header_model
            .update(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to update header: {}", e)))?;

        Ok(result.into())
    } else {
        // Insert the new header entry
        let header_model: game_header::ActiveModel = local_header.into();
        let result = header_model
            .insert(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to insert header: {}", e)))?;

        Ok(result.into())
    }
}
