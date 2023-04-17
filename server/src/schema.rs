use async_graphql::{Context, Object};
use sea_orm::*;

use crate::entities::{prelude::*, *};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn contacts(&self, ctx: &Context<'_>) -> Result<Vec<contacts::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Contacts::find().all(db).await
    }


}