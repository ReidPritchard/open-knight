use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::gametag::Entity",
        from = "Column::Id",
        to = "super::gametag::Column::TagId"
    )]
    GameTags,
    #[sea_orm(
        has_many = "super::move_tag::Entity",
        from = "Column::Id",
        to = "super::move_tag::Column::TagId"
    )]
    MoveTags,
}

impl Related<super::gametag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameTags.def()
    }
}

impl Related<super::move_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MoveTags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
