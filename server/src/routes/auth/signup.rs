use actix_web::{web, HttpResponse};
use anyhow::Context;
use secrecy::{Secret, ExposeSecret};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::auth::compute_password_hash;
use crate::domain::UserEmail;
use crate::email_client::EmailClient;
use crate::error::{RegisterError, TokenError};
use crate::startup::ApplicationBaseUrl;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::generate_token;

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

    let password      = json.password.clone();
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;
        
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

    // Will error locally w/o email configured
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
async fn insert_user(
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
async fn store_token(
    transaction:        &mut Transaction<'_, Postgres>,
    user_id:            Uuid,
    confirmation_token: &str
) -> Result<(), TokenError> {
    sqlx::query!(
        r#"
        INSERT INTO confirmation_tokens (confirmation_token, user_id)
        VALUES ($1, $2)
        "#,
        confirmation_token,
        user_id
    ).execute(transaction)
    .await
    .map_err(TokenError::DatabaseError)?;

    Ok(())
}

#[tracing::instrument(
    name = "Send a confirmation email to a new user",
    skip(email_client, email, base_url, token)
)]
async fn send_confirmation_email(
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