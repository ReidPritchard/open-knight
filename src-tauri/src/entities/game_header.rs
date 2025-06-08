use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "game_header")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub header_id: i32,
    pub game_id: i32,
    pub header_name: String,
    pub header_value: String,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::game::Entity",
        from = "Column::GameId",
        to = "super::game::Column::GameId"
    )]
    Game,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
