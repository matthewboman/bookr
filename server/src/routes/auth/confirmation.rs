use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::TokenError;

#[derive(serde::Deserialize)]
pub struct Parameters {
    token: String,
}

#[tracing::instrument(
    name = "Confirm a pending user",
    skip(parameters, pool)
)]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool:       web::Data<PgPool>
) -> Result<HttpResponse, TokenError> { 
    let user_id = get_user_id_from_token(&pool, &parameters.token)
        .await
        .context("Failed to find a user with the provided confirmation token")?
        .ok_or(TokenError::InvalidToken)?;

    confirm_user(&pool, user_id)
        .await
        .context("Failed to update user status to `confirmed`.")?;
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Get a user_id from a token",
    skip(token, pool)
)]
async fn get_user_id_from_token(
    pool:  &PgPool, 
    token: &str
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT user_id FROM confirmation_tokens WHERE confirmation_token = $1",
        token
    ).fetch_optional(pool)
    .await?;

    Ok(result.map(|r| r.user_id))
}

#[tracing::instrument(
    name = "Mark user as confirmed",
    skip(user_id, pool)
)]
async fn confirm_user(
    pool:    &PgPool, 
    user_id: Uuid
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET status = 'confirmed' WHERE user_id = $1"#,
        user_id
    ).execute(pool)
    .await?;

    Ok(())
}