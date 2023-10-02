use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::{
    add_contact_genre_relation,
    NewContact,
    OptionalStringInput, 
    StringInput,   
};
use crate::error::ContentError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContactData {
    pub display_name: String,
    pub address:      Option<String>,
    pub city:         String,
    pub state:        Option<String>,
    pub zip_code:     Option<String>,
    pub capacity:     Option<i32>,
    pub email:        Option<String>,
    pub contact_form: Option<String>,
    pub age_range:    String,
    pub is_private:   bool,
    pub contact_type: String,
    pub genres:       Vec<i32>,
}

impl TryFrom<NewContactData> for NewContact {
    type Error = String;

    fn try_from(value: NewContactData) -> Result<Self, Self::Error> {
        let display_name = StringInput::parse(value.display_name);
        let address      = OptionalStringInput::parse(value.address);
        let city         = StringInput::parse(value.city);
        let state        = OptionalStringInput::parse(value.state);
        let zip_code     = OptionalStringInput::parse(value.zip_code);
        let email        = OptionalStringInput::parse(value.email);
        let contact_form = OptionalStringInput::parse(value.contact_form);
        let age_range    = StringInput::parse(value.age_range);
        let contact_type = StringInput::parse(value.contact_type);

        Ok(Self {
            display_name,
            address,
            city,
            state,
            zip_code,
            capacity: value.capacity,
            email,
            contact_form,
            age_range,
            contact_type,
            is_private: value.is_private,
            genres:     value.genres
        })
    }
}

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

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new contact to database",
    skip(contact, transaction, user_id)
)]
pub async fn insert_contact(
    contact:     &NewContact,
    transaction: &mut Transaction<'_, Postgres>,
    user_id:     &Uuid,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO contacts (
            display_name, address, city, state, zip_code, capacity, email, 
            contact_form, age_range, is_private, contact_type,
            user_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING contact_id
        "#,
        contact.display_name,
        contact.address,
        contact.city,
        contact.state,
        contact.zip_code,
        contact.capacity,
        contact.email,
        contact.contact_form,
        contact.age_range,
        contact.is_private,
        contact.contact_type,
        user_id
    ).fetch_one(transaction)
    .await?;

    let contact_id: i32 = result.contact_id;

    Ok(contact_id)
}