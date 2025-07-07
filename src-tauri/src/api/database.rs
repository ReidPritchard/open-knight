use crate::entities::*;
use crate::models::{ChessGame, ChessMove, ChessPosition};
use ok_utils::ts_export;
use sea_orm::QueryFilter;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QuerySelect,
    Select,
};
use std::collections::HashMap;

#[ts_export]
pub struct QueryParams {
    #[ts(type = "number")]
    pub limit: Option<u64>,
    #[ts(type = "number")]
    pub offset: Option<u64>,
    pub fields: Option<Vec<String>>,
    pub filter: Option<HashMap<String, String>>,
    pub load_moves: Option<bool>,
    pub load_tags: Option<bool>,
    pub load_headers: Option<bool>,
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            limit: Some(100),
            offset: Some(0),
            fields: None,
            filter: None,
            load_moves: Some(false),
            load_tags: Some(false),
            load_headers: Some(false),
        }
    }
}

#[ts_export]
pub struct QueryResult {
    #[ts(type = "string[]")]
    pub data: Vec<serde_json::Value>,
    pub total: usize,
}

/// Trait for mapping string field names to entity columns
pub trait ColumnMapper {
    type Column: ColumnTrait;

    fn get_column(field: &str) -> Option<Self::Column>;
}

impl ColumnMapper for game::Entity {
    type Column = game::Column;

    fn get_column(field: &str) -> Option<Self::Column> {
        match field {
            "id" => Some(game::Column::GameId),
            "white_player_id" => Some(game::Column::WhitePlayerId),
            "black_player_id" => Some(game::Column::BlackPlayerId),
            "result" => Some(game::Column::Result),
            "tournament_id" => Some(game::Column::TournamentId),
            "opening_id" => Some(game::Column::OpeningId),
            "round_number" => Some(game::Column::RoundNumber),
            "date_played" => Some(game::Column::DatePlayed),
            "fen" => Some(game::Column::Fen),
            "pgn" => Some(game::Column::Pgn),
            "created_at" => Some(game::Column::CreatedAt),
            _ => None,
        }
    }
}

impl ColumnMapper for player::Entity {
    type Column = player::Column;

    fn get_column(field: &str) -> Option<Self::Column> {
        match field {
            "id" => Some(player::Column::PlayerId),
            "name" => Some(player::Column::Name),
            "elo" => Some(player::Column::EloRating),
            "title" => Some(player::Column::Title),
            "country_code" => Some(player::Column::CountryCode),
            "created_at" => Some(player::Column::CreatedAt),
            _ => None,
        }
    }
}

impl ColumnMapper for tournament::Entity {
    type Column = tournament::Column;

    fn get_column(field: &str) -> Option<Self::Column> {
        match field {
            "id" => Some(tournament::Column::TournamentId),
            "name" => Some(tournament::Column::Name),
            "type" => Some(tournament::Column::Type),
            "time_control" => Some(tournament::Column::TimeControl),
            "start_date" => Some(tournament::Column::StartDate),
            "end_date" => Some(tournament::Column::EndDate),
            "location" => Some(tournament::Column::Location),
            _ => None,
        }
    }
}

impl ColumnMapper for r#move::Entity {
    type Column = r#move::Column;

    fn get_column(field: &str) -> Option<Self::Column> {
        match field {
            "id" => Some(r#move::Column::MoveId),
            "game_id" => Some(r#move::Column::GameId),
            "ply_number" => Some(r#move::Column::PlyNumber),
            "san" => Some(r#move::Column::San),
            "uci" => Some(r#move::Column::Uci),
            "position_id" => Some(r#move::Column::PositionId),
            "created_at" => Some(r#move::Column::CreatedAt),
            _ => None,
        }
    }
}

impl ColumnMapper for opening::Entity {
    type Column = opening::Column;

    fn get_column(field: &str) -> Option<Self::Column> {
        match field {
            "id" => Some(opening::Column::OpeningId),
            "eco_code" => Some(opening::Column::EcoCode),
            "name" => Some(opening::Column::Name),
            "variation" => Some(opening::Column::Variation),
            _ => None,
        }
    }
}

impl ColumnMapper for tag::Entity {
    type Column = tag::Column;

    fn get_column(field: &str) -> Option<Self::Column> {
        match field {
            "id" => Some(tag::Column::TagId),
            "name" => Some(tag::Column::Name),
            "description" => Some(tag::Column::Description),
            _ => None,
        }
    }
}

fn apply_filters<E: EntityTrait + ColumnMapper>(
    filter: Option<&HashMap<String, String>>
) -> Condition {
    let mut condition = Condition::all();

    if let Some(filters) = filter {
        for (field, value) in filters {
            if let Some(column) = E::get_column(field) {
                condition = condition.add(column.eq(value.to_owned()));
            }
        }
    }

    condition
}

fn select_fields<E: EntityTrait + ColumnMapper>(
    query: Select<E>,
    fields: Option<&Vec<String>>,
) -> Select<E> {
    if let Some(fields) = fields {
        let columns: Vec<_> =
            fields.iter().filter_map(|f| E::get_column(f)).collect();

        if !columns.is_empty() {
            return query.select_only().columns(columns);
        }
    }
    query
}

pub async fn query_entities(
    entity: &str,
    params: QueryParams,
    db: &DatabaseConnection,
) -> anyhow::Result<QueryResult> {
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);

    let result = match entity {
        "games" => {
            let condition =
                apply_filters::<game::Entity>(params.filter.as_ref());
            select_fields(game::Entity::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "players" => {
            let condition =
                apply_filters::<player::Entity>(params.filter.as_ref());
            select_fields(player::Entity::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "tournaments" => {
            let condition =
                apply_filters::<tournament::Entity>(params.filter.as_ref());
            select_fields(tournament::Entity::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "moves" => {
            let condition =
                apply_filters::<r#move::Entity>(params.filter.as_ref());
            select_fields(r#move::Entity::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "openings" => {
            let condition =
                apply_filters::<opening::Entity>(params.filter.as_ref());
            select_fields(opening::Entity::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "tags" => {
            let condition =
                apply_filters::<tag::Entity>(params.filter.as_ref());
            select_fields(tag::Entity::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        _ => return Err(anyhow::anyhow!("Unknown entity type: {}", entity)),
    };

    Ok(QueryResult {
        data: result.clone(),
        total: result.len(),
    })
}

pub async fn get_entity_by_id(
    entity: &str,
    id: i32,
    fields: Option<Vec<String>>,
    db: &DatabaseConnection,
) -> anyhow::Result<Option<serde_json::Value>> {
    let result = match entity {
        "games" => {
            select_fields(game::Entity::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "players" => {
            select_fields(player::Entity::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "tournaments" => {
            select_fields(tournament::Entity::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "moves" => {
            select_fields(r#move::Entity::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "openings" => {
            select_fields(opening::Entity::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "tags" => {
            select_fields(tag::Entity::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        _ => return Err(anyhow::anyhow!("Unknown entity type: {}", entity)),
    };

    Ok(result)
}

pub async fn get_full_game(
    game_id: i32,
    params: QueryParams,
    db: &DatabaseConnection,
) -> anyhow::Result<Option<ChessGame>> {
    let mut game = match ChessGame::load(db, game_id).await {
        Ok(game) => game,
        Err(_) => return Ok(None),
    };

    if params.load_moves.unwrap_or(false) {
        let _ = game.load_moves(db).await;
    }

    if params.load_tags.unwrap_or(false) {
        let _ = game.load_tags(db).await;
    }

    if params.load_headers.unwrap_or(false) {
        let _ = game.load_headers(db).await;
    }

    Ok(Some(game))
}

pub async fn query_full_games(
    params: QueryParams,
    db: &DatabaseConnection,
) -> anyhow::Result<Vec<ChessGame>> {
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);
    let condition = apply_filters::<game::Entity>(params.filter.as_ref());

    // First get the game IDs
    let game_ids: Vec<i32> = game::Entity::find()
        .filter(condition)
        .limit(limit)
        .offset(offset)
        .all(db)
        .await?
        .into_iter()
        .map(|g| g.game_id)
        .collect();

    // Then load full games
    let mut games = Vec::new();
    for id in game_ids {
        if let Ok(mut game) = ChessGame::load(db, id).await {
            if params.load_moves.unwrap_or(false) {
                let _ = game.load_moves(db).await;
            }
            if params.load_tags.unwrap_or(false) {
                let _ = game.load_tags(db).await;
            }
            if params.load_headers.unwrap_or(false) {
                let _ = game.load_headers(db).await;
            }
            games.push(game);
        }
    }

    Ok(games)
}

/**
 * Get all moves for a specific game
 */
pub async fn get_game_moves(
    game_id: i32,
    db: &DatabaseConnection,
) -> anyhow::Result<Vec<ChessMove>> {
    use sea_orm::LoaderTrait;

    let db_moves = r#move::Entity::find()
        .filter(r#move::Column::GameId.eq(game_id))
        .all(db)
        .await?;

    let move_positions = db_moves.load_one(position::Entity, db).await?;

    let moves = db_moves
        .into_iter()
        .zip(move_positions.into_iter())
        .map(|(curr_move, curr_position)| {
            let move_position = curr_position.map_or(
                ChessPosition {
                    id: 0,
                    fen: "".to_string(),
                    evaluations: Vec::new(),
                    variant: None,
                },
                |p| ChessPosition {
                    id: p.position_id,
                    fen: p.fen,
                    evaluations: Vec::new(),
                    variant: None,
                },
            );

            ChessMove {
                id: curr_move.move_id,
                game_id,
                ply_number: curr_move.ply_number,
                san: curr_move.san,
                uci: curr_move.uci,
                position: Some(move_position),
                annotations: Vec::new(),
                time_info: None,
                parent_move_id: curr_move.parent_move_id,
            }
        })
        .collect::<Vec<ChessMove>>();

    Ok(moves)
}
