use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "movetimetracking")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub move_id: i32,
    pub time_spent_ms: Option<i32>,
    pub time_left_ms: Option<i32>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::move_::Entity",
        from = "Column::MoveId",
        to = "super::move_::Column::Id"
    )]
    Move,
}

impl Related<super::move_::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Move.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
