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
        SELECT c.contact_id, c.display_name, c.address, c.city, c.state, 
               c.zip_code, c.capacity, c.latitude, c.longitude, c.email, 
               c.contact_form, c.age_range, c.country, c.is_private, c.user_id,
               ROUND(AVG(r.rating), 2)::real AS average_rating
        FROM contacts c
        LEFT JOIN reviews r ON c.contact_id = r.contact_id
        WHERE c.is_private = true
        AND c.verified = $1
        AND c.user_id = $2
        GROUP BY c.contact_id
        "#,
        verified,
        user_id
    ).fetch_all(pool)
    .await?;

    Ok(contacts)
}