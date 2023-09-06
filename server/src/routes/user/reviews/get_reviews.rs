use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::query_reviews_by_user;
use crate::error::ContentError;

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn user_get_reviews(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let reviews = query_reviews_by_user(user_id, &pool)
        .await
        .context("Failed to get reviews for user")?;
    
    Ok(HttpResponse::Ok().json(reviews))
}