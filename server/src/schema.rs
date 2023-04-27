use async_graphql::{Context, Object, EmptyMutation, EmptySubscription, Schema};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::entities::{prelude::*, *};

#[derive(Debug)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn contacts(&self, ctx: &Context<'_>) -> Result<Vec<contacts::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Contacts::find().all(db).await
    }
}

pub type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
