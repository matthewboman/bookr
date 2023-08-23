use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::{delete_review, query_reviews, query_reviews_by_user, Review};
use crate::error::AdminError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewData {
    review_id: Uuid,
    user_id:   Uuid,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    user_id: Uuid
}

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn admin_delete_review(
    req:  HttpRequest,
    json: web::Json<ReviewData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, AdminError> {
    let ext     = req.extensions();
    let role    = ext.get::<String>().unwrap();
    let admin   = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    delete_review(&json.review_id, &json.user_id, &pool)
        .await
        .context("Failed to delete review")?;

    // TODO: send an email to user saying review has been deleted
    // probably need a policy or selectable reason

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn admin_get_all_reviews(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, AdminError> {
    let ext     = req.extensions();
    let role    = ext.get::<String>().unwrap();
    let admin   = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    let reviews = query_reviews(&pool)
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
    let ext     = req.extensions();
    let role    = ext.get::<String>().unwrap();
    let admin   = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    let reviews = query_reviews_by_user(&params.user_id, &pool)
        .await
        .context("Failed to get reviews for user")?;
    
    Ok(HttpResponse::Ok().json(reviews))
}