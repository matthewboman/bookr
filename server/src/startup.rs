use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, get_contacts};

pub fn run(
    listener: TcpListener,
    db_pool:  PgPool,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server  = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .app_data(db_pool.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/contacts", web::get().to(get_contacts))
    })
    .listen(listener)?
    .run();

    Ok(server)
}