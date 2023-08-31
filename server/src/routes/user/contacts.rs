use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::contact::{
    delete_contact, update_contact,
    ContactResponse, EditContactData, EditedContact
};
use crate::error::ContentError;
use crate::utils::user_matches;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteData {
    user_id:    Uuid,
    contact_id: i32,
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
    let verified = true; // TODO: user's might want a list of their pending contacts. Created `pending_private_contacts` when building edit workflow.
 
    let contacts = query_private_contacts(user_id, verified, &pool)
        .await
        .context("Failed to get private contacts from database")?;
    
    Ok(HttpResponse::Ok().json(contacts))    
}

#[tracing::instrument(
    skip(req, pool, json)
)]
pub async fn user_delete_contact(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    json: web::Json<DeleteData>,
    _:    JwtMiddleware
) -> Result<HttpResponse, ContentError> {
    let ext      = req.extensions();
    let user_id  = ext.get::<uuid::Uuid>().unwrap();

    user_matches(user_id, &json.user_id)
        .context("User IDs don't match when deleting review")?;
    delete_contact(&json.contact_id, &pool)
        .await
        .context("Failed to delete contact")?;
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, pool, json)
)]
pub async fn user_edit_contact(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    json: web::Json<EditContactData>,
    _:    JwtMiddleware
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    user_matches(user_id, &json.user_id)
        .context("User IDs don't match when updating contact")?;
    
    let contact: EditedContact = json.0.try_into().map_err(ContentError::ValidationError)?;

    update_contact(contact, &pool)
        .await
        .context("Failed to update contact")?;

    // TODO: if verified && address changes, update latitute and longitude
    // TODO: need someway to handle if this process errors. new column `erroring` && view for admin?
    Ok(HttpResponse::Ok().finish())
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