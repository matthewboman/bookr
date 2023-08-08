use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::contact::ContactResponse;
use crate::error::ContactError;

#[tracing::instrument(
    skip(pool),
)]
pub async fn public_contacts(pool: web::Data<PgPool>) -> Result<HttpResponse, ContactError> {
    let contacts = query_contacts(&pool)
        .await
        .context("Failed to query contacts for guest")?;
     
    Ok(HttpResponse::Ok().json(contacts))   
}

#[tracing::instrument(
    name = "Querying contacts from DB",
    skip(pool)
)]
async fn query_contacts(pool: &PgPool) -> Result<Vec<ContactResponse>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        ContactResponse,
        r#"
        SELECT contact_id, display_name, address, city, state, zip_code, capacity, latitude, longitude, email, contact_form, age_range, country, is_private, user_id
        FROM contacts
        WHERE is_private = false
        AND verified = true
        "#
    ).fetch_all(pool)
    .await?;

    Ok(contacts)
}