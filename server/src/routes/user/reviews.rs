use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::input_validator::OptionalStringInput;
use crate::error::ContentError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonData {
    contact_id: i32,
    title:      Option<String>,
    body:       Option<String>,
    rating:     i32,
}

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn review_contact(
    req:  HttpRequest,
    json: web::Json<JsonData>,
    pool: web::Data<PgPool>,
    _: JwtMiddleware
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    // TODO: no longer needs to be a transaction
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    
    verify_contact_exists(json.contact_id.clone(), &mut transaction)
        .await
        .context("Failed to find Contact with provided contact_id")?;

    let review_id = insert_review(&json, user_id, &mut transaction)
        .await
        .context("Failed to insert new review into database")?;
    
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to add new review")?;
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Verifying contact exists for review",
    skip(contact_id, transaction)
)]
async fn verify_contact_exists(
    contact_id:  i32,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        SELECT contact_id FROM contacts WHERE contact_id = $1
        "#,
        contact_id
    ).fetch_one(transaction)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Saving new review to database",
    skip(review, transaction)
)]
async fn insert_review(
    review:      &web::Json<JsonData>,
    user_id:     &Uuid,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<Uuid, sqlx::Error> {
    let title = OptionalStringInput::parse(&review.title);
    let body  = OptionalStringInput::parse(&review.body);

    let review = sqlx::query!(
        r#"
        INSERT INTO reviews (user_id, contact_id, title, body, rating)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING review_id
        "#,
        user_id,
        review.contact_id,
        title.as_deref(),
        body.as_deref(),
        review.rating
    )
    .fetch_one(transaction)
    .await?;

    let review_id: Uuid = review.review_id;

    Ok(review_id)
}