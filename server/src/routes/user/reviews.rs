use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};

use crate::auth::JwtMiddleware;
use crate::domain::input_validator::StringInput;
use crate::domain::{
    delete_review, edit_review, insert_review, query_reviews_by_user, 
    NewReview, Review, ReviewDeleteData, ReviewEditData
};
use crate::error::ContentError;
use crate::utils::user_matches;

// TODO: refactor
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateData {
    contact_id: i32,
    title:      String,
    body:       String,
    rating:     i32
}

impl TryFrom<CreateData> for NewReview {
    type Error = String;

    fn try_from(value: CreateData) -> Result<Self, Self::Error> {
        let title = StringInput::parse(value.title);
        let body  = StringInput::parse(value.body);

        Ok(Self {
            title, 
            body,
            contact_id: value.contact_id,
            rating: value.rating
        })
    }
}

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn review_contact(
    req:  HttpRequest,
    json: web::Json<CreateData>,
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

    insert_review(review, user_id, &mut transaction)
        .await
        .context("Failed to insert new review into database")?;
    
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to add new review")?;
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn user_delete_review(
    req:  HttpRequest,
    json: web::Json<ReviewDeleteData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    user_matches(user_id, &json.user_id)?;
    delete_review(&json.review_id, user_id, &pool)
        .await
        .context("Failed to delete review")?;
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, json, pool)
)]
pub async fn user_edit_review(
    req:  HttpRequest,
    json: web::Json<ReviewEditData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let review: Review = json.0.try_into().map_err(ContentError::ValidationError)?;

    user_matches(user_id, &review.user_id)
        .context("User IDs don't match when editing review")?;
    edit_review(review, &pool)
        .await
        .context("Failed to update review")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn user_get_reviews(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let reviews = query_reviews_by_user(user_id, &pool)
        .await
        .context("Failed to get reviews for user")?;
    
    Ok(HttpResponse::Ok().json(reviews))
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