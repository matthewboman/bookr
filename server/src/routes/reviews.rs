use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::{ContactReview, Review};
use crate::error::ContentError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    contact_id: i32
}

#[tracing::instrument(
    skip(pool, params)
)]
pub async fn reviews_for_contact(
    params: web::Query<Params>,
    pool:   web::Data<PgPool>
) -> Result<HttpResponse, ContentError> {
    let reviews = query_reviews_for_contact(&pool, &params.contact_id)
        .await
        .context("Failed to return reviews for the associated contact")?;
    let response = average_reviews(params.contact_id, reviews);
    
    Ok(HttpResponse::Ok().json(response))
}

#[tracing::instrument(
    skip(pool, contact_id)
)]
async fn query_reviews_for_contact(
    pool:       &PgPool,
    contact_id: &i32,
) -> Result<Vec<Review>, sqlx::Error> {
    let result = sqlx::query_as!(
        Review,
        r#"
        SELECT review_id, contact_id, user_id, title, body, rating
        FROM reviews
        WHERE contact_id = $1
        "#,
        contact_id
    ).fetch_all(pool)
    .await?;

    Ok(result)
}

#[tracing::instrument(
    skip(contact_id, reviews)
)]
fn average_reviews(
    contact_id: i32,
    reviews:    Vec<Review>
) -> ContactReview {
    let mut ratings: Vec<i32> = Vec::new();

    for review in reviews.iter() {
        ratings.push(review.rating);
    }

    let rating = ratings.iter().sum::<i32>() as f32 / ratings.len() as f32;

    ContactReview {
        contact_id,
        rating,
        reviews
    }
}