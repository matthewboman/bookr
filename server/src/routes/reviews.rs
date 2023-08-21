use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::{ContactReview, ReviewResponse};
use crate::error::ContactError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    contact_id: i64
}

#[tracing::instrument(
    skip(pool, params)
)]
pub async fn reviews_for_contact(
    params: web::Query<Params>,
    pool:   web::Data<PgPool>
) -> Result<HttpResponse, ContactError> {
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
    contact_id: &i64,
) -> Result<Vec<ReviewResponse>, sqlx::Error> {
    let result = sqlx::query_as!(
        ReviewResponse,
        r#"
        SELECT review_id, title, body, rating
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
    contact_id: i64,
    reviews:    Vec<ReviewResponse>
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