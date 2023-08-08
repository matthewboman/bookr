use actix_web::{web, HttpResponse};
use anyhow::Context;
use secrecy::{Secret, ExposeSecret};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::{compute_password_hash};
use crate::domain::UserEmail;
use crate::domain::input_validator::StringInput;
use crate::email_client::EmailClient;
use crate::error::TokenError;
use crate::startup::ApplicationBaseUrl;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::generate_token;

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
    let reset_token = StringInput::parse(json.reset_token.expose_secret().to_string());
    let user_id     = get_user_id_from_reset_token(&pool, reset_token)
        .await
        .context("Unable to find user with provided token")?
        .ok_or(TokenError::InvalidToken)?;

    // TODO: server-side validation to test new passwords

    let password      = json.new_password.clone();
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;

    update_password(&pool, user_id, password_hash)
        .await
        .context("Failed to update password in database")?;

    Ok(HttpResponse::Ok().finish())
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
async fn get_user_id_from_reset_token(
    pool:        &PgPool,
    reset_token: String,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT user_id FROM reset_tokens WHERE reset_token = $1"#,
        reset_token
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| r.user_id))
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
    skip(pool, uuid, new_password)
)]
async fn update_password(
    pool:         &PgPool,
    uuid:         Uuid,
    new_password: Secret<String>
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET password_hash = $1 WHERE user_id = $2
        "#,
        new_password.expose_secret(),
        uuid
    ).execute(pool)
    .await?;

    Ok(())
}