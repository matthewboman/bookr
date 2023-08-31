use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::contact::{update_contact, EditContactData, EditedContact};
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
    
    update_contact(contact, &pool)
        .await
        .context("Failed to update contact")?;

    // TODO: if verified && address changes, update latitute and longitude
    // TODO: need someway to handle if this process errors. new column `erroring` && view for admin?

    Ok(HttpResponse::Ok().finish())
}