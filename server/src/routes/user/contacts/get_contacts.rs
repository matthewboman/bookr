use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::{format_contact_response, ContactResponse, ContactRow};
use crate::error::ContentError;

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn user_get_contacts(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, ContentError> {
    let ext      = req.extensions();
    let user_id  = ext.get::<uuid::Uuid>().unwrap();
    let private  = false;
    let contacts = query_user_contacts(user_id, private, &pool)
        .await
        .context("Failed to get private contacts from database")?;

    Ok(HttpResponse::Ok().json(contacts))
}

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
    let private  = true;
    let contacts = query_user_contacts(user_id, private, &pool)
        .await
        .context("Failed to get private contacts from database")?;
    
    Ok(HttpResponse::Ok().json(contacts))    
}

#[tracing::instrument(
    name = "Querying User's contacts from DB",
    skip(pool, user_id, private)
)]
async fn query_user_contacts(
    user_id:  &Uuid,
    private:  bool,
    pool:     &PgPool
) -> Result<Vec<ContactResponse>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        ContactRow,
        r#"
        SELECT c.contact_id, c.display_name, c.address, c.city, c.state, 
               c.zip_code, c.capacity, c.latitude, c.longitude, c.email, 
               c.contact_form, c.age_range, c.country, c.is_private, c.contact_type,
               c.user_id,
               ROUND(AVG(r.rating), 2)::real AS average_rating,
               g.genre_name, g.genre_id
        FROM contacts c
        LEFT JOIN reviews r ON c.contact_id = r.contact_id
        LEFT JOIN contacts_genres ON c.contact_id = contacts_genres.contact_id
        LEFT JOIN genres g on g.genre_id = contacts_genres.genre_id
        WHERE c.is_private = true OR c.is_private = $1
        AND c.user_id = $2
        GROUP BY c.contact_id, g.genre_name, g.genre_id
        "#, // `is_private` is intentionally redundant to allow for `true` or all
        private,
        user_id
    ).fetch_all(pool)
    .await?;

    let result = format_contact_response(contacts);

    Ok(result)
}