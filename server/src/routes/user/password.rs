use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use secrecy::Secret;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::{validate_credentials, AuthError, Credentials, JwtMiddleware};
use crate::utils::e500;

#[derive(serde::Deserialize)]
pub struct PasswordResetData {
    current_password: Secret<String>,
    new_password:     Secret<String>,
}

pub async fn change_password(
    req:  HttpRequest,
    json: web::Json<PasswordResetData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, actix_web::Error> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let email   = get_email(*user_id, &pool).await.map_err(e500)?;

    let credentials = Credentials {
        email,
        password: json.0.current_password
    };

    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                Ok(
                    HttpResponse::Unauthorized().finish()
                )
            }
            AuthError::UnexpectedError(_) => Err(e500(e))
        };
    }

    crate::auth::change_password(*user_id, json.0.new_password, &pool)
        .await
        .map_err(e500)?;
    
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