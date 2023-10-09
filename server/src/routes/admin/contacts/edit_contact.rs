use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::contact::{
    query_contact_by_id,
    update_contact,
    update_contact_genres,
    EditContactData, 
    EditedContact
};
use crate::error::AdminError;
use crate::utils::is_admin;

#[tracing::instrument(
    skip(req, pool, json)
)]
pub async fn admin_edit_contact(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    json: web::Json<EditContactData>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AdminError> {
    is_admin(req)?;

    let contact: EditedContact = json.0.try_into().map_err(AdminError::ValidationError)?;
    
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    update_contact(&contact, &mut transaction)
        .await
        .context("Failed to update contact")?;

    update_contact_genres(&contact, &mut transaction)
        .await
        .context("Failed to update genres for contact")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to update contact")?;

    // If this fails, the transaction shouldn't fail
    let contact = query_contact_by_id(&pool, &contact.contact_id)
        .await
        .context("Failed to query updated contact from the database")?;

    // TODO: if verified && address changes, update latitute and longitude
    // TODO: need someway to handle if this process errors. new column `erroring` && view for admin?
    Ok(HttpResponse::Ok().json(contact))
}