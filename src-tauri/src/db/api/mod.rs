use sea_orm::{
    sea_query::Expr, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QuerySelect, Select,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{entities::*, models::ChessGame};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub fields: Option<Vec<String>>,
    pub filter: Option<HashMap<String, String>>,
    pub load_moves: Option<bool>,
    pub load_tags: Option<bool>,
}

pub struct QueryResult {
    pub data: serde_json::Value,
    pub total: usize,
}

fn apply_filters(filter: Option<&HashMap<String, String>>) -> Condition {
    let mut condition = Condition::all();

    if let Some(filters) = filter {
        for (field, value) in filters {
            condition = condition.add(Expr::col(field).eq(value.clone()));
        }
    }

    condition
}

fn select_fields<E: EntityTrait>(query: Select<E>, fields: Option<&Vec<String>>) -> Select<E> {
    if let Some(fields) = fields {
        let columns = fields.iter().map(|f| Expr::col(f)).collect::<Vec<_>>();
        query.select_only().columns(columns)
    } else {
        query
    }
}

pub async fn query_entities(
    entity: &str,
    params: QueryParams,
    db: &DatabaseConnection,
) -> anyhow::Result<QueryResult> {
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);
    let condition = apply_filters(params.filter.as_ref());

    let result = match entity {
        "games" => {
            select_fields(Game::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "players" => {
            select_fields(Player::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "tournaments" => {
            select_fields(Tournament::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "moves" => {
            select_fields(Move::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "openings" => {
            select_fields(Opening::find(), params.fields.as_ref())
                .filter(condition)
                .limit(limit)
                .offset(offset)
                .into_json()
                .all(db)
                .await?
        }
        "tags" => {
            select_fields(Tag::find(), params.fields.as_ref())
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
        data: serde_json::json!(result),
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
            select_fields(Game::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "players" => {
            select_fields(Player::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "tournaments" => {
            select_fields(Tournament::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "moves" => {
            select_fields(Move::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "openings" => {
            select_fields(Opening::find_by_id(id), fields.as_ref())
                .into_json()
                .one(db)
                .await?
        }
        "tags" => {
            select_fields(Tag::find_by_id(id), fields.as_ref())
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

    Ok(Some(game))
}

pub async fn query_full_games(
    params: QueryParams,
    db: &DatabaseConnection,
) -> anyhow::Result<Vec<ChessGame>> {
    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);
    let condition = apply_filters(params.filter.as_ref());

    // First get the game IDs
    let game_ids: Vec<i32> = Game::find()
        .filter(condition)
        .limit(limit)
        .offset(offset)
        .all(db)
        .await?
        .into_iter()
        .map(|g| g.id)
        .collect();

    // Then load full games
    let mut games = Vec::new();
    for id in game_ids {
        if let Some(mut game) = ChessGame::load(db, id).await.ok() {
            if params.load_moves.unwrap_or(false) {
                let _ = game.load_moves(db).await;
            }
            if params.load_tags.unwrap_or(false) {
                let _ = game.load_tags(db).await;
            }
            games.push(game);
        }
    }

    Ok(games)
}
