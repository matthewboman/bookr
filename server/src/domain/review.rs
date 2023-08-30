use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::StringInput;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewReview {
    pub contact_id: i32,
    pub title:      String,
    pub body:       String,
    pub rating:     i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub review_id:  Uuid,
    pub contact_id: i32,
    pub user_id:    Uuid,
    pub title:      String,
    pub body:       String,
    pub rating:     i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactReview {
    pub contact_id: i32,
    pub rating:     f32,
    pub reviews:    Vec<Review>
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewDeleteData {
    pub review_id: Uuid,
    pub user_id:   Uuid,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewEditData {
    contact_id: i32,
    user_id:    Uuid,
    review_id:  Uuid,
    title:      String,
    body:       String,
    rating:     i32
}

impl TryFrom<ReviewEditData> for Review {
    type Error = String;

    fn try_from(value: ReviewEditData) -> Result<Self, Self::Error> {
        let title = StringInput::parse(value.title);
        let body  = StringInput::parse(value.body);

        Ok(Self {
            title, 
            body,
            contact_id: value.contact_id,
            rating: value.rating,
            user_id: value.user_id,
            review_id: value.review_id
        })
    }
}

#[tracing::instrument(
    skip(pool, review_id)
)]
pub async fn delete_review(
    review_id: &Uuid,
    user_id:   &Uuid,
    pool:      &PgPool,
) -> Result<(), sqlx::Error> {
    println!("deleting {} by {}", &review_id, &user_id);
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
    skip(pool, review)
)]
pub async fn edit_review(
    review:  Review,
    pool:    &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE reviews
        SET title = $1, body = $2, rating = $3
        WHERE review_id = $4
        "#,
        review.title,
        review.body,
        review.rating,
        review.review_id,
    ).execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Saving new review to database",
    skip(review, transaction)
)]
pub async fn insert_review(
    review:      NewReview,
    user_id:     &Uuid,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
// ) -> Result<Uuid, sqlx::Error> {
    // let review = sqlx::query!(
    sqlx::query!(
        r#"
        INSERT INTO reviews (user_id, contact_id, title, body, rating)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING review_id
        "#,
        user_id,
        review.contact_id,
        review.title,
        review.body,
        review.rating
    )
    .fetch_one(transaction)
    .await?;

    // let review_id: Uuid = review.review_id;

    // Ok(review_id)
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