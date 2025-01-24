use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "game")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub white_player_id: i32,
    pub white_player_elo: Option<i32>,
    pub black_player_id: i32,
    pub black_player_elo: Option<i32>,
    pub tournament_id: Option<i32>,
    pub opening_id: Option<i32>,
    pub result: String,
    pub round_number: Option<i32>,
    pub date_played: String,
    pub fen: Option<String>,
    pub pgn: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::WhitePlayerId",
        to = "super::player::Column::Id"
    )]
    WhitePlayer,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::BlackPlayerId",
        to = "super::player::Column::Id"
    )]
    BlackPlayer,
    #[sea_orm(
        belongs_to = "super::tournament::Entity",
        from = "Column::TournamentId",
        to = "super::tournament::Column::Id"
    )]
    Tournament,
    #[sea_orm(
        belongs_to = "super::opening::Entity",
        from = "Column::OpeningId",
        to = "super::opening::Column::Id"
    )]
    Opening,
    #[sea_orm(
        has_many = "super::move_::Entity",
        from = "Column::Id",
        to = "super::move_::Column::GameId"
    )]
    Moves,
    #[sea_orm(
        has_many = "super::gametag::Entity",
        from = "Column::Id",
        to = "super::gametag::Column::GameId"
    )]
    GameTags,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WhitePlayer.def()
    }
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl Related<super::opening::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Opening.def()
    }
}

impl Related<super::move_::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Moves.def()
    }
}

impl Related<super::gametag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameTags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
