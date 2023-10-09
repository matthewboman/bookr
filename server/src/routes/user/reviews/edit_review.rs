use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::{
    edit_review, Review, ReviewEditData
};
use crate::error::ContentError;
use crate::utils::user_matches;

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn user_edit_review(
    req:  HttpRequest,
    json: web::Json<ReviewEditData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let review: Review = json.0.try_into().map_err(ContentError::ValidationError)?;

    user_matches(user_id, &review.user_id)
        .context("User IDs don't match when editing review")?;
    let review = edit_review(review, &pool)
        .await
        .context("Failed to update review")?;

    Ok(HttpResponse::Ok().json(review))
}