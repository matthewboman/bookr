use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::{PgPool, postgres::{PgQueryResult}};
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::contact::{Contact, NewContact};

#[tracing::instrument(
    skip(req, json, pool),
)]
pub async fn add_contact(
    req:  HttpRequest,
    json: web::Json<NewContact>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> HttpResponse {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    // Do I need to verify that the user exists?

    let contact = match insert_contact(json, &pool, user_id).await {
        Ok(c) => c,
        Err(e) => {
            println!("error inserting contact {}", e);

            return HttpResponse::InternalServerError().finish()
        }
    };

    // println!("New contact{}", contact.contact_id);

    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Saving new contact to database",
    skip(contact, pool, user_id)
)]
pub async fn insert_contact(
    contact: web::Json<NewContact>,
    pool:    &PgPool,
    user_id: &Uuid,
// ) -> Result<Contact, sqlx::Error> {
) -> Result<PgQueryResult, sqlx::Error> {
    let contact = sqlx::query!(
        r#"
        INSERT INTO contacts (display_name, address, city, state, zip_code, capacity, email, contact_form, age_range, is_private, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        &contact.display_name,
        &contact.address,
        &contact.city,
        &contact.state,
        &contact.zip_code,
        &contact.capacity,
        &contact.email,
        &contact.contact_form,
        &contact.age_range,
        &contact.is_private,
        user_id
    ).execute(pool)
    .await?;

    Ok(contact)
}