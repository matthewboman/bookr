use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::contact::delete_contact;
use crate::error::ContentError;
use crate::utils::user_matches;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteData {
    user_id:    Uuid,
    contact_id: i32,
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

    user_matches(user_id, &json.user_id)?;
    delete_contact(&json.contact_id, &pool)
        .await
        .context("Failed to delete contact")?;
    
    Ok(HttpResponse::Ok().finish())
}