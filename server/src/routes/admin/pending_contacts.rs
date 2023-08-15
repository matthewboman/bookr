use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::{PgPool, postgres::{PgQueryResult}};

use crate::auth::JwtMiddleware;
use crate::domain::PendingContact;
use crate::error::AdminError;

// #[tracing::instrument(
//     skip(req, json, pool),
// )]
// pub async fn approve_contact(
//     req:  HttpRequest,
//     json: web::Json<JsonData>,
//     pool: web::Data<PgPool>,
//     _:    JwtMiddleware
// ) -> Result<HttpResponse, ResponseError> {
//     todo!()
// }

#[tracing::instrument(
    skip(req, pool),
)]
pub async fn pending_contacts(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AdminError> {
    let ext   = req.extensions();
    let role  = ext.get::<String>().unwrap();
    let admin = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    let contacts = query_pending_contacts(&pool)
        .await
        .context("Failed to query pending contacts for admin")?;
    
    Ok(HttpResponse::Ok().json(contacts))
}

#[tracing::instrument(
    name = "Getting all pending contacts from the database",
    skip(pool)
)]
pub async fn query_pending_contacts(
    pool: &PgPool
) -> Result<Vec<PendingContact>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        PendingContact,
        r#"
        SELECT contact_id, display_name, address, city, state, zip_code, capacity, email, contact_form, age_range, country, user_id
        FROM contacts
        WHERE verified = false
        "#
    ).fetch_all(pool)
    .await?;

    Ok(contacts)
}