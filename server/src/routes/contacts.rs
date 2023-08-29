use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::ContactResponse;
use crate::error::ContentError;

#[tracing::instrument(
    skip(pool),
)]
pub async fn public_contacts(pool: web::Data<PgPool>) -> Result<HttpResponse, ContentError> {
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
        SELECT c.contact_id, c.display_name, c.address, c.city, c.state, c.zip_code, 
               c.capacity, c.latitude, c.longitude, c.email, c.contact_form, 
               c.age_range, c.country, c.is_private, c.user_id, ROUND(AVG(r.rating), 2)::real AS average_rating
        FROM contacts c
        LEFT JOIN reviews r ON c.contact_id = r.contact_id
        WHERE c.is_private = false
        AND c.verified = true
        GROUP BY c.contact_id
        "#
    ).fetch_all(pool)
    .await?;

    Ok(contacts)
}