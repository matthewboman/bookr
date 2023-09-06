use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::Duration;
use secrecy::{Secret, ExposeSecret};
use sqlx::{PgPool, Postgres, Transaction, types::chrono::NaiveDateTime};
use uuid::Uuid;

use crate::auth::{compute_password_hash};
use crate::domain::UserEmail;
use crate::domain::input_validator::StringInput;
use crate::email_client::EmailClient;
use crate::error::TokenError;
use crate::startup::ApplicationBaseUrl;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::{generate_token, is_token_expired};

#[derive(serde::Deserialize)]
pub struct ResetRequest {
    email: String
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordData {
    reset_token:        Secret<String>,
    new_password:       Secret<String>,
    new_password_check: Secret<String>
}

#[tracing::instrument(
    skip(json, pool, email_client, base_url),
    fields(
        email=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn generate_reset_token(
    json:         web::Json<ResetRequest>,
    pool:         web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url:     web::Data<ApplicationBaseUrl>
) -> Result<HttpResponse, TokenError> {
    let email   = UserEmail::parse(json.email.clone())
        .map_err(TokenError::ValidationError)?;
    let user_id = get_user_id_from_email(&pool, &email)
        .await
        .context("Failed to find user with provided email")?
        .ok_or(TokenError::UnknownEmail)?;

    let reset_token = generate_token();
    store_token(&pool, &user_id, &reset_token)
        .await
        .context("Failed to insert token")?;

    // This won't work in local environment. Make sure to comment out.
    send_password_reset_email(
        &email_client,
        email,
        &base_url.0,
        &reset_token
    ).await
    .context("Failed to send password reset email")?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn reset_password(
    json: web::Json<ResetPasswordData>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, TokenError> {
    let reset_token    = StringInput::parse(json.reset_token.expose_secret().to_string());
    let token_duration = Duration::minutes(60);
    let (user_id, exp) = get_details_from_reset_token(&pool, &reset_token)
        .await
        .context("Unable to find user with provided token")?
        .ok_or(TokenError::InvalidToken)?;
    
    if is_token_expired(exp, token_duration) {
        return Err(TokenError::InvalidToken)
    }

    if json.new_password.expose_secret() != json.new_password_check.expose_secret() {
        return Err(TokenError::ValidationError("Passwords do not match".to_string()))
    }

    let password      = json.new_password.clone();
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    update_password(&mut transaction, user_id, password_hash)
        .await
        .context("Failed to update password in database")?;

    delete_token(&mut transaction, user_id, reset_token)
        .await
        .context("Failed to delete password reset token")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to reset user's password")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Deleting token from database",
    skip(transaction, uuid, reset_token)
)]
async fn delete_token(
    transaction: &mut Transaction<'_, Postgres>,
    uuid:        Uuid,
    reset_token: String
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM reset_tokens WHERE user_id = $1 AND reset_token = $2
        "#,
        uuid,
        reset_token
    ).execute(transaction)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Verifying user email exists",
    skip(email, pool)
)]
async fn get_user_id_from_email(
    pool:  &PgPool, 
    email: &UserEmail
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT user_id FROM users WHERE email = $1"#,
        email.as_ref()
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| r.user_id))
}

#[tracing::instrument(
    name = "Looking up user_id from reset_token",
    skip(reset_token, pool)
)]
async fn get_details_from_reset_token(
    pool:        &PgPool,
    reset_token: &String,
) -> Result<Option<(Uuid, NaiveDateTime)>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT user_id, created_at FROM reset_tokens WHERE reset_token = $1"#,
        reset_token
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| (r.user_id, r.created_at)))
}

#[tracing::instrument(
    name = "Send a password reset email",
    skip(email_client, email, base_url, reset_token)
)]
async fn send_password_reset_email(
    email_client: &EmailClient,
    email:        UserEmail,
    base_url:     &str,
    reset_token:  &str
) -> Result<(), reqwest::Error> {
    let reset_link = format!(
        "{}/reset-password?reset_token={}",
        base_url, reset_token
    );

    let plain_body = &format!(
        "Your password reset request has been received. Visit {} to reset your password",
        reset_link
    );
    let html_body = &format!(
        "Your password reset request has been received. <br />Click <a href=\"{}\">here</a> to reset your password",
        reset_link
    );

    email_client.send_email(
        &email,
        "Password reset",
        &plain_body,
        &html_body
    ).await
}

#[tracing::instrument(
    name = "Store password reset token in the database",
    skip(reset_token, pool)
)]
async fn store_token(
    pool:        &PgPool,
    user_id:     &Uuid,
    reset_token: &String
) -> Result<(), TokenError> {
    sqlx::query!(
        r#"
        INSERT INTO reset_tokens (user_id, reset_token)
        VALUES ($1, $2)
        "#,
        user_id,
        reset_token
    ).execute(pool)
    .await
    .map_err(TokenError::DatabaseError)?;

    Ok(())
}

#[tracing::instrument(
    name = "Update user password hash in database",
    skip(transaction, uuid, new_password)
)]
async fn update_password(
    transaction:  &mut Transaction<'_, Postgres>,
    uuid:         Uuid,
    new_password: Secret<String>
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET password_hash = $1 WHERE user_id = $2
        "#,
        new_password.expose_secret(),
        uuid
    ).execute(transaction)
    .await?;

    Ok(())
}