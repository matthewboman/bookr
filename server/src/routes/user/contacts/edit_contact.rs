use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::contact::{
    update_contact, EditContactData, EditedContact
};
use crate::error::ContentError;
use crate::utils::user_matches;

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

    user_matches(user_id, &json.user_id)?;
    
    let contact: EditedContact = json.0.try_into().map_err(ContentError::ValidationError)?;

    update_contact(contact, &pool)
        .await
        .context("Failed to update contact")?;

    // TODO: if verified && address changes, update latitute and longitude
    // TODO: need someway to handle if this process errors. new column `erroring` && view for admin?
    Ok(HttpResponse::Ok().finish())
}