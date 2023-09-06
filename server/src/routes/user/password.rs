use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::{validate_credentials, AuthError, Credentials, JwtMiddleware};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetData {
    current_password:   Secret<String>,
    new_password:       Secret<String>,
    new_password_check: Secret<String>,
}

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn change_password(
    req:  HttpRequest,
    json: web::Json<PasswordResetData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AuthError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let email   = get_email(*user_id, &pool)
        .await
        .context("Failed to find user with provided email")?;

    if json.new_password.expose_secret() != json.new_password_check.expose_secret() {
        return Err(AuthError::ValidationError("Passwords do not match".to_string()))
    }

    let credentials = Credentials {
        email,
        password: json.0.current_password
    };

    validate_credentials(credentials, &pool).await?;

    crate::auth::change_password(*user_id, json.0.new_password, &pool)
        .await
        .context("Failed to update password")?;
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Get email",
    skip(pool)
)]
async fn get_email(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(r#"SELECT email FROM users WHERE user_id = $1"#, user_id)
        .fetch_one(pool)
        .await
        .context("Failed to perform a query to retrieve a email")?;

    Ok(row.email)
}