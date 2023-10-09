use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};

use crate::auth::JwtMiddleware;
use crate::domain::{insert_review, CreateReviewData, NewReview};
use crate::error::ContentError;

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn review_contact(
    req:  HttpRequest,
    json: web::Json<CreateReviewData>,
    pool: web::Data<PgPool>,
    _: JwtMiddleware
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let review: NewReview = json.0.try_into().map_err(ContentError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    
    verify_contact_exists(&review.contact_id, &mut transaction)
        .await
        .context("Failed to find Contact with provided contact_id")?;

    let review = insert_review(review, user_id, &mut transaction)
        .await
        .context("Failed to insert new review into database")?;
    
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to add new review")?;
    
    Ok(HttpResponse::Ok().json(review))
}

#[tracing::instrument(
    name = "Verifying contact exists for review",
    skip(contact_id, transaction)
)]
async fn verify_contact_exists(
    contact_id:  &i32,
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