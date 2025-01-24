use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "move")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub game_id: i32,
    pub parent_move_id: Option<i32>,
    pub move_number: Option<i32>,
    pub player_color: Option<String>,
    pub move_notation: String,
    pub position_id: Option<i32>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::game::Entity",
        from = "Column::GameId",
        to = "super::game::Column::Id"
    )]
    Game,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentMoveId",
        to = "Column::Id",
        on_delete = "NoAction"
    )]
    ParentMove,
    #[sea_orm(
        belongs_to = "super::position::Entity",
        from = "Column::PositionId",
        to = "super::position::Column::Id"
    )]
    Position,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl Related<super::position::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Position.def()
    }
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParentMove.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
