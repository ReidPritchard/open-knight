use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "position")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub fen: String,
    pub fen_hash: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::move_::Entity",
        from = "Column::Id",
        to = "super::move_::Column::PositionId"
    )]
    Moves,
    #[sea_orm(
        has_many = "super::evaluation::Entity",
        from = "Column::Id",
        to = "super::evaluation::Column::PositionId"
    )]
    Evaluations,
}

impl Related<super::move_::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Moves.def()
    }
}

impl Related<super::evaluation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Evaluations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
