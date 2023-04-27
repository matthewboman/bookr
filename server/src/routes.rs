use actix_web::{web, HttpResponse, Result};
use async_graphql::{http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::schema::SchemaType;

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=uft-8")
            .body(GraphiQLSource::build().endpoint("/graphql").finish())
    )
}

pub async fn graphql(schema: web::Data<SchemaType>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}