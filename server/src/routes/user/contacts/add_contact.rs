use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::{
    add_contact_genre_relation,
    insert_contact,
    query_contact_by_id,
    NewContact,
    NewContactData
};
use crate::error::ContentError;

#[tracing::instrument(
    skip(req, json, pool),
)]
pub async fn add_contact(
    req:  HttpRequest,
    json: web::Json<NewContactData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let contact: NewContact = json.0.try_into().map_err(ContentError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let contact_id = insert_contact(&contact, &mut transaction, user_id)
        .await
        .context("Failed to insert new contact into database")?;

    add_contact_genre_relation(&contact_id, contact.genres, &mut transaction)
        .await
        .context("Failed to insert contacts_genres relation")?;
    
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to add new Contact")?;

    // If this fails, the transaction shouldn't fail
    let contact = query_contact_by_id(&pool, &contact_id)
        .await
        .context("Failed to query newly added contact from database")?;

    Ok(HttpResponse::Ok().json(contact))
}