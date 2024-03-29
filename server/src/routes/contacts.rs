use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::{
    format_contact_response,
    query_contact_by_id,
    ContactResponse, 
    ContactRow
};
use crate::error::ContentError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactParams {
    contact_id: i32,
}

#[tracing::instrument(
    skip(pool, params),
)]
pub async fn get_contact_by_id(
    params: web::Query<ContactParams>,
    pool:   web::Data<PgPool>,
) -> Result<HttpResponse, ContentError> {
    let contact = query_contact_by_id(&pool, &params.contact_id)
        .await
        .context("Failed to query contacts for guest")?;
 
    Ok(HttpResponse::Ok().json(contact))
}

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
        ContactRow,
        r#"
        SELECT
            c.contact_id, c.display_name, c.address, c.city, c.state, c.zip_code, c.country,
            c.capacity, c.latitude, c.longitude, c.email, c.contact_form, c.age_range,
            c.user_id, c.is_private, c.contact_type,
            ROUND(AVG(r.rating), 2)::real AS average_rating,
            g.genre_name, g.genre_id
        FROM contacts c
        LEFT JOIN reviews r ON c.contact_id = r.contact_id
        LEFT JOIN contacts_genres ON c.contact_id = contacts_genres.contact_id
        LEFT JOIN genres g ON g.genre_id = contacts_genres.genre_id
        WHERE c.is_private = false
        GROUP BY
            c.contact_id, g.genre_name, g.genre_id
        "#
    )
    .fetch_all(pool)
    .await?;

    let result = format_contact_response(contacts);

    Ok(result)
}