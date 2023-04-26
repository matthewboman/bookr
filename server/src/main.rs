mod entities;
mod routes;
mod schema;
mod setup;

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use routes::{graphiql, graphql};
use schema::*;
use setup::connect_to_db;

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