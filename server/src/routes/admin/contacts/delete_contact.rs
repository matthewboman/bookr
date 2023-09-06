use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::delete_contact;
use crate::error::AdminError;
use crate::utils::is_admin;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteData {
    contact_id: i32,
    _user_id:    Uuid
}

#[tracing::instrument(
    skip(req, json, pool),
)]
pub async fn admin_delete_contact(
    req:  HttpRequest,
    json: web::Json<DeleteData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AdminError> {
    is_admin(req)?;
    delete_contact(&json.contact_id, &pool)
        .await
        .context("Failed to delete contact")?;
    
    Ok(HttpResponse::Ok().finish())
}