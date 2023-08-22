use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::Review;
use crate::error::AdminError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewData {
    review_id: Uuid
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

    delete_review(&json.review_id, &pool)
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

    let reviews = query_reviews_by_user(params.user_id, &pool)
        .await
        .context("Failed to get reviews for user")?;
    
    Ok(HttpResponse::Ok().json(reviews))
}

#[tracing::instrument(
    skip(pool, review_id)
)]
async fn delete_review(
    review_id: &Uuid,
    pool:      &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM reviews WHERE review_id = $1
        "#,
        review_id
    ).execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    skip(pool, user_id)
)]
async fn query_reviews_by_user(
    user_id: Uuid,
    pool:    &PgPool
) -> Result<Vec<Review>, sqlx::Error> {
    let reviews = sqlx::query_as!(
        Review,
        r#"
        SELECT review_id, user_id, contact_id, title, body, rating
        FROM reviews
        where user_id = $1
        "#,
        user_id
    ).fetch_all(pool)
    .await?;

    Ok(reviews)
}

#[tracing::instrument(
    skip(pool)
)]
async fn query_reviews(
    pool:    &PgPool
) -> Result<Vec<Review>, sqlx::Error> {
    let reviews = sqlx::query_as!(
        Review,
        r#"
        SELECT review_id, user_id, contact_id, title, body, rating
        FROM reviews
        "#
    ).fetch_all(pool)
    .await?;

    Ok(reviews)
}