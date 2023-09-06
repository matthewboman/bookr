use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::{
    delete_review, ReviewDeleteData
};
use crate::error::ContentError;
use crate::utils::user_matches;

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn user_delete_review(
    req:  HttpRequest,
    json: web::Json<ReviewDeleteData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    user_matches(user_id, &json.user_id)?;
    delete_review(&json.review_id, user_id, &pool)
        .await
        .context("Failed to delete review")?;
    
    Ok(HttpResponse::Ok().finish())
}