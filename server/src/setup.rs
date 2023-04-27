use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use actix_web::dev::Server;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::extensions::Tracing;
use sea_orm::DatabaseConnection;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::{graphiql, graphql};
use crate::schema::*;

pub fn run(
    listener: TcpListener,
    db_pool:  DatabaseConnection,
) -> Result<Server, std::io::Error> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db_pool)
        .extension(Tracing)
        .finish();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(web::resource("/graphiql").guard(guard::Get()).to(graphiql))
    })
    .listen(listener)?
    .run();

    Ok(server)
}