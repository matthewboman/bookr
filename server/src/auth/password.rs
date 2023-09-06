use actix_web::{ResponseError, http::StatusCode};
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{Secret, ExposeSecret};
use sqlx::PgPool;

use crate::domain::{CleanUser, User};
use crate::telemetry::spawn_blocking_with_tracing;

pub struct Credentials {
    pub email:    String,
    pub password: Secret<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("{0}")]
    ValidationError(String),
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(credentials, pool)
)]
pub async fn validate_credentials(
    credentials: Credentials,
    pool:        &PgPool
) -> Result<CleanUser, AuthError> {
    let mut user = None;
    // Default hashed password if no user is found to prevent timing attacks
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some(stored_user) = get_stored_credentials(
        &credentials.email,
        &pool
    ).await? {
        expected_password_hash = stored_user.password_hash;
        user = Some(CleanUser {
            user_id: stored_user.user_id,
            email:   stored_user.email,
            role:    stored_user.role
        });
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    }).await.context("Failed to spawn blocking task")??;

    user
        .ok_or_else(|| anyhow::anyhow!("Unknown email"))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Veridy password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate:     Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format")?;
    
    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash
        ).context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Get stored credentials",
    skip(email, pool)
)]
async fn get_stored_credentials(
    email: &str,
    pool:  &PgPool
) -> Result<Option<User>, anyhow::Error> {
    let user = sqlx::query!(
        r#"SELECT user_id, password_hash, role FROM users WHERE email = $1"#,
        email,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to retrieve stored credentials.")?
    .map(|row| User {
        user_id: row.user_id,
        email: email.to_string(),
        password_hash: row.password_hash.into(),
        role: row.role
    });

    Ok(user)
}

#[tracing::instrument(
    name = "Change password",
    skip(password, pool)
)]
pub async fn change_password(
    user_id:  uuid::Uuid,
    password: Secret<String>,
    pool:     &PgPool,
) -> Result<(), anyhow::Error> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;
    
    sqlx::query!(
        r#"UPDATE users SET password_hash = $1 WHERE user_id = $2"#,
        password_hash.expose_secret(),
        user_id
    ).execute(pool)
    .await
    .context("Failed to change user's password in database")?;

    Ok(())
}

pub fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap()
    ).hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();

    Ok(Secret::new(password_hash))
}