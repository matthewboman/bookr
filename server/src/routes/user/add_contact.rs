use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::{PgPool, postgres::{PgQueryResult}};
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::input_validator::{OptionalStringInput, StringInput};
use crate::error::ContentError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonData {
    pub display_name: String,
    pub address:      Option<String>,
    pub city:         String,
    pub state:        Option<String>,
    pub zip_code:     Option<String>,
    pub capacity:     Option<i32>,
    pub email:        Option<String>,
    pub contact_form: Option<String>,
    pub age_range:    Option<String>,
    pub is_private:   bool,
}

#[tracing::instrument(
    skip(req, json, pool),
)]
pub async fn add_contact(
    req:  HttpRequest,
    json: web::Json<JsonData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, ContentError> {
    let ext     = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    insert_contact(json, &pool, user_id)
        .await
        .context("Failed to insert new contact into database")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new contact to database",
    skip(contact, pool, user_id)
)]
pub async fn insert_contact(
    contact: web::Json<JsonData>,
    pool:    &PgPool,
    user_id: &Uuid,
) -> Result<PgQueryResult, sqlx::Error> {
    let display_name = StringInput::parse(contact.display_name.clone());
    let address      = OptionalStringInput::parse(&contact.address);
    let city         = StringInput::parse(contact.city.clone());
    let state        = OptionalStringInput::parse(&contact.state);
    let zip_code     = OptionalStringInput::parse(&contact.zip_code);
    let email        = OptionalStringInput::parse(&contact.email);
    let contact_form = OptionalStringInput::parse(&contact.contact_form);
    let age_range    = OptionalStringInput::parse(&contact.age_range);

    let contact = sqlx::query!(
        r#"
        INSERT INTO contacts (display_name, address, city, state, zip_code, capacity, email, contact_form, age_range, is_private, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        display_name,
        address.as_deref(),
        city,
        state.as_deref(),
        zip_code.as_deref(),
        contact.capacity.as_ref().map(|&c| c),
        email.as_deref(),
        contact_form.as_deref(),
        age_range.as_deref(),
        contact.is_private,
        user_id
    ).execute(pool)
    .await?;

    Ok(contact)
}