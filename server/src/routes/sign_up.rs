use actix_web::{web, HttpResponse, ResponseError, http::StatusCode};
use anyhow::Context;
use secrecy::{Secret, ExposeSecret};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::auth::{compute_password_hash};
use crate::domain::UserEmail;
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::{error_chain_fmt, generate_token};

#[derive(serde::Deserialize)]
pub struct JsonData {
    email:    String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(json, pool, email_client, base_url),
    fields(
        email=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn sign_up(
    json:         web::Json<JsonData>,
    pool:         web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url:     web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, RegisterError> {
    let email = UserEmail::parse(json.email.clone())
        .map_err(RegisterError::ValidationError)?;

    // TODO: use error chain
    let password      = json.password.clone();
    let password_hash = match spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await
        .context("Failed to hash password")?
    {
        Ok(s) => s,
        Err(_e) => {
            return Ok(HttpResponse::InternalServerError().finish())
        }
    };
        
    let mut transaction = pool.begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    
    let user_id = insert_user(&mut transaction, email.as_ref(), password_hash)
        .await
        .context("A user with the provided email already exists")?;

    let confirmation_token = generate_token();

    store_token(&mut transaction, user_id, &confirmation_token)
        .await
        .context("Failed to store the confirmation token")?;

    transaction.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber")?;

    send_confirmation_email(
        &email_client,
        email,
        &base_url.0,
        &confirmation_token
    ).await
    .context("Failed to send confirmation email")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new user to the database",
    skip(email, password_hash, transaction)
)]
pub async fn insert_user(
    transaction:   &mut Transaction<'_, Postgres>,
    email:         &str,
    password_hash: Secret<String>
) -> Result<Uuid, sqlx::Error> {
    let user_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO users (user_id, email, password_hash)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        email,
        password_hash.expose_secret()
    ).execute(transaction)
    .await?;

    Ok(user_id)
}

#[tracing::instrument(
    name = "Store confirmation token in the database",
    skip(confirmation_token, transaction)
)]
pub async fn store_token(
    transaction:        &mut Transaction<'_, Postgres>,
    user_id:            Uuid,
    confirmation_token: &str
) -> Result<(), StoreTokenError> {
    sqlx::query!(
        r#"
        INSERT INTO confirmation_tokens (confirmation_token, user_id)
        VALUES ($1, $2)
        "#,
        confirmation_token,
        user_id
    ).execute(transaction)
    .await
    .map_err(StoreTokenError)?;

    Ok(())
}

#[tracing::instrument(
    name = "Send a confirmation email to a new user",
    skip(email_client, email, base_url, token)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    email:        UserEmail,
    base_url:     &str,
    token:        &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!("{}/confirm?token={}", base_url, token);

    let plain_body = &format!(
        "Welcome to Book Your Own Tour. Visit {} to confirm your account",
        confirmation_link
    );
    let html_body = &format!(
        "Welcome to Book Your Own Tour. <br />Click <a href=\"{}\">here</a> to confirm your account",
        confirmation_link
    );

    email_client.send_email(
        &email,
        "Confirm your account",
        &plain_body,
        &html_body
    ).await
}

#[derive(thiserror::Error)]
pub enum RegisterError {
    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for RegisterError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegisterError::ValidationError(_) => StatusCode::BAD_REQUEST,
            RegisterError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub struct StoreTokenError(sqlx::Error);

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a subscription token"
        )
    }
}

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}