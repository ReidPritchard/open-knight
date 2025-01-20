use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "player")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub name: String,
    pub last_known_elo: Option<i32>,
    pub country: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::game::Entity",
        from = "Column::Id",
        to = "super::game::Column::WhitePlayerId"
    )]
    WhiteGames,
    #[sea_orm(
        has_many = "super::game::Entity",
        from = "Column::Id",
        to = "super::game::Column::BlackPlayerId"
    )]
    BlackGames,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WhiteGames.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
