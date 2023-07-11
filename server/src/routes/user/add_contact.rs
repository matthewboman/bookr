use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::contact::NewContact;

#[tracing::instrument(
    skip(req, json, pool),
)]
pub async fn add_contact(
    req:  HttpRequest,
    json: web::Json<NewContact>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> HttpResponse {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    println!("{}", json.display_name);
    println!("added by {}", user_id);

    HttpResponse::Ok().finish()
}