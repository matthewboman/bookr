use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::{
    delete_review, edit_review, query_recent_reviews, query_reviews_by_user, 
    Review, ReviewEditData, ReviewDeleteData,
    UserParams
};
use crate::error::AdminError;
use crate::utils::is_admin;

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn admin_delete_review(
    req:  HttpRequest,
    json: web::Json<ReviewDeleteData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, AdminError> {
    is_admin(req)?;
    delete_review(&json.review_id, &json.user_id, &pool)
        .await
        .context("Failed to delete review")?;

    // TODO: send an email to user saying review has been deleted
    // probably need a policy or selectable reason

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn admin_edit_review(
    req:  HttpRequest,
    json: web::Json<ReviewEditData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, AdminError> {
    is_admin(req)?;

    let review: Review = json.0.try_into().map_err(AdminError::ValidationError)?;

    edit_review(review, &pool)
        .await
        .context("Failed to update review")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn admin_get_recent_reviews(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, AdminError> {
    is_admin(req)?;

    let reviews = query_recent_reviews(&pool)
        .await
        .context("Failed to get reviews")?;
    
    Ok(HttpResponse::Ok().json(reviews))
}

#[tracing::instrument(
    skip(req, params, pool)
)]
pub async fn admin_get_reviews_by_user(
    req:    HttpRequest,
    params: web::Query<UserParams>,
    pool:   web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, AdminError> {
    is_admin(req)?;

    let reviews = query_reviews_by_user(&params.user_id, &pool)
        .await
        .context("Failed to get reviews for user")?;
    
    Ok(HttpResponse::Ok().json(reviews))
}