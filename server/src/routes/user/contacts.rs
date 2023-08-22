use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::contact::ContactResponse;
use crate::error::ContentError;

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn private_contacts(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext      = req.extensions();
    let user_id  = ext.get::<uuid::Uuid>().unwrap();
    let verified = true; // TODO: user's might want a list of their pending contacts. Created `pending_private_contacts` when building edit workflow.
 
    let contacts = query_private_contacts(user_id, verified, &pool)
        .await
        .context("Failed to get private contacts from database")?;
    
    Ok(HttpResponse::Ok().json(contacts))    
}

#[tracing::instrument(
    name = "Querying private contacts from DB",
    skip(pool, user_id, verified)
)]
async fn query_private_contacts(
    user_id:  &Uuid,
    verified: bool,
    pool:     &PgPool
) -> Result<Vec<ContactResponse>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        ContactResponse,
        r#"
        SELECT contact_id, display_name, address, city, state, zip_code, capacity, latitude, longitude, email, contact_form, age_range, country, is_private, user_id
        FROM contacts
        WHERE is_private = true
        AND verified = $1
        AND user_id = $2
        "#,
        verified,
        user_id
    ).fetch_all(pool)
    .await?;

    Ok(contacts)
}