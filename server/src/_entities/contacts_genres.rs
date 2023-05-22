use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "contacts_genres"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub contact_id: i32,
    pub genre_id:   i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    ContactId,
    GenreId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    ContactId,
    GenreId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Contacts,
    Genres,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::ContactId  => ColumnType::Integer.def(),
            Self::GenreId => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Contacts => Entity::belongs_to(super::contacts::Entity)
                .from(Column::ContactId)
                .to(super::contacts::Column::ContactId)
                .into(),
            Self::Genres => Entity::belongs_to(super::genres::Entity)
                .from(Column::GenreId)
                .to(super::genres::Column::GenreId)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}