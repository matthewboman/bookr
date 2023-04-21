use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

// TODO: Should I distinguish between contact types: venues, buyers/promoters?

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(name = "Contacts")]
#[sea_orm(table_name = "contacts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub contact_id:   i32,
    pub display_name: String,
    pub address:      Option<String>,
    pub city:         String,
    pub state:        Option<String>,
    pub zip_code:     Option<String>,
    pub country:      Option<String>,
    pub latitude:     Option<f32>,
    pub longitude:    Option<f32>,
    pub capacity:     Option<i32>, // TODO: Some venues have multiple rooms with different capacity....
    pub email:        Option<String>,
    pub contact_form: Option<String>,
    pub age_range:    Option<String>,

    pub created_at:   DateTime,
    pub updated_at:   DateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::genres::Entity> for Entity {
    fn to() -> RelationDef {
        super::contacts_genres::Relation::Genres.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::contacts_genres::Relation::Contacts.def().rev())
    }
}