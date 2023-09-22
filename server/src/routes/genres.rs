use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::Genre;
use crate::error::ContentError;
use crate::redis_cli::{get_data_as_json, store_data_as_json};

#[tracing::instrument(
    skip(pool)
)]
pub async fn get_genres(
    pool:  web::Data<PgPool>,
    redis: web::Data<redis::Client>
) -> Result<HttpResponse, ContentError> {
    let mut conn = redis.get_tokio_connection()
        .await
        .context("Could not reg async Redis connection")?;
    let cached_genres: Option<Vec<Genre>> = get_data_as_json(&mut conn, "genres")
        .await
        .context("Could not get cached genres from Redis")?;

    if let Some(cached_genres) = cached_genres {
        return Ok(HttpResponse::Ok().json(cached_genres));
    }

    let genres = query_genres(&pool)
        .await
        .context("Failed to query list of genres")?;

    store_data_as_json(&mut conn, "genres", &genres)
        .await
        .context("Could not store genres in Redis")?;

    Ok(HttpResponse::Ok().json(genres))
}

#[tracing::instrument(
    name = "Querying genres from DB",
    skip(pool)
)]
async fn query_genres(pool: &PgPool) -> Result<Vec<Genre>, sqlx::Error> {
    let genres = sqlx::query_as!(
        Genre,
        r#"
        SELECT genre_id, genre_name FROM genres
        "#
    ).fetch_all(pool)
    .await?;

    Ok(genres)
}