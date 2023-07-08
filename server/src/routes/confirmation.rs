use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

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
) -> HttpResponse {
    let user_id = match get_user_id_from_token(&pool, &parameters.token).await {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    match user_id {
        None => HttpResponse::Unauthorized().finish(),
        Some(user_id) => {
            if confirm_user(&pool, user_id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
    }
}

#[tracing::instrument(
    name = "Get a user_id from a token",
    skip(token, pool)
)]
pub async fn get_user_id_from_token(pool: &PgPool, token: &str) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT user_id FROM confirmation_tokens WHERE confirmation_token = $1",
        token
    ).fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;

    Ok(result.map(|r| r.user_id))
}

#[tracing::instrument(
    name = "Mark user as confirmed",
    skip(user_id, pool)
)]
pub async fn confirm_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET status = 'confirmed' WHERE user_id = $1"#,
        user_id
    ).execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;

    Ok(())
}