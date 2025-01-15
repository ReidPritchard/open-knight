use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "evaluation")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub position_id: i32,
    pub evaluation_score: Option<f32>,
    pub evaluation_type: Option<String>,
    pub depth: Option<i32>,
    pub engine_name: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::position::Entity",
        from = "Column::PositionId",
        to = "super::position::Column::Id"
    )]
    Position,
}

impl Related<super::position::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Position.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
