use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::Genre;
use crate::error::ContentError;

#[tracing::instrument(
    skip(pool)
)]
pub async fn get_genres(pool: web::Data<PgPool>) -> Result<HttpResponse, ContentError> {
    let genres = query_genres(&pool)
        .await
        .context("Failed to query list of genres")?;
    
    // TODO: implement Redis caching

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