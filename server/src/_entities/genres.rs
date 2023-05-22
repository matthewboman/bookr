use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(name = "Genres")]
#[sea_orm(table_name = "genres")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub genre_id:   i32,
    pub name:       String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::contacts::Entity> for Entity {
    fn to() -> RelationDef {
        super::contacts_genres::Relation::Contacts.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::contacts_genres::Relation::Genres.def().rev())
    }
}