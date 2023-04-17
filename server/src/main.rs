mod entities;
mod schema;
mod setup;
// mod types;

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::*;

use schema::*;
use setup::connect_to_db;

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphiql() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=uft-8")
            .body(GraphiQLSource::build().endpoint("/").finish())
    )
}

async fn graphql(schema: web::Data<SchemaType>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match connect_to_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err)
    };

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(crate::web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql))
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}