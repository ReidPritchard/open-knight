use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "opening")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub eco_code: Option<String>,
    pub name: Option<String>,
    pub variation: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::game::Entity",
        from = "Column::Id",
        to = "super::game::Column::OpeningId"
    )]
    Games,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Games.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
