use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Review {
    pub review_id:  Uuid,
    pub contact_id: i32,
    pub user_id:    Uuid,
    pub title:      Option<String>,
    pub body:       Option<String>,
    pub rating:     i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResponse {
    pub review_id:  Uuid,
    pub title:      Option<String>,
    pub body:       Option<String>,
    pub rating:     i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactReview {
    pub contact_id: i32,
    pub rating:     f32,
    pub reviews:    Vec<ReviewResponse>
}

#[tracing::instrument(
    skip(pool, review_id)
)]
pub async fn delete_review(
    review_id: &Uuid,
    user_id:   &Uuid,
    pool:      &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM reviews WHERE review_id = $1 AND user_id = $2
        "#,
        review_id,
        user_id
    ).execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    skip(pool)
)]
pub async fn query_reviews(
    pool:    &PgPool
) -> Result<Vec<Review>, sqlx::Error> {
    let reviews = sqlx::query_as!(
        Review,
        r#"
        SELECT review_id, user_id, contact_id, title, body, rating
        FROM reviews
        "#
    ).fetch_all(pool)
    .await?;

    Ok(reviews)
}

#[tracing::instrument(
    skip(pool, user_id)
)]
pub async fn query_reviews_by_user(
    user_id: &Uuid,
    pool:    &PgPool
) -> Result<Vec<Review>, sqlx::Error> {
    let reviews = sqlx::query_as!(
        Review,
        r#"
        SELECT review_id, user_id, contact_id, title, body, rating
        FROM reviews
        where user_id = $1
        "#,
        user_id
    ).fetch_all(pool)
    .await?;

    Ok(reviews)
}