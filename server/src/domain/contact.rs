use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{OptionalStringInput, StringInput};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub contact_id:   i32,
    pub display_name: String,
    pub address:      Option<String>,
    pub city:         String,
    pub state:        Option<String>,
    pub zip_code:     Option<String>,
    pub country:      Option<String>,
    pub latitude:     Option<f32>,
    pub longitude:    Option<f32>,
    pub capacity:     Option<i32>,
    pub email:        Option<String>,
    pub contact_form: Option<String>,
    pub age_range:    String,
    pub user_id:      Uuid,
    pub is_private:   bool,
    pub verified:     bool,
    pub created_at:   chrono::DateTime<chrono::Utc>,
    pub updated_at:   chrono::DateTime<chrono::Utc>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactResponse {
    pub contact_id:     i32,
    pub display_name:   String,
    pub address:        Option<String>,
    pub city:           String,
    pub state:          Option<String>,
    pub zip_code:       Option<String>,
    pub country:        Option<String>,
    pub latitude:       Option<f32>,
    pub longitude:      Option<f32>,
    pub capacity:       Option<i32>,
    pub email:          Option<String>,
    pub contact_form:   Option<String>,
    pub age_range:      String,
    pub user_id:        Uuid,
    pub is_private:     bool,
    pub average_rating: Option<f32>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditContactData {
    pub contact_id:   i32,
    pub display_name: String,
    pub address:      Option<String>,
    pub city:         String,
    pub state:        Option<String>,
    pub zip_code:     Option<String>,
    pub country:      Option<String>,
    pub capacity:     Option<i32>,
    pub email:        Option<String>,
    pub contact_form: Option<String>,
    pub age_range:    String,
    pub user_id:      Uuid,
    pub is_private:   bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditedContact {
    pub contact_id:   i32,
    pub display_name: String,
    pub address:      Option<String>,
    pub city:         String,
    pub state:        Option<String>,
    pub zip_code:     Option<String>,
    pub country:      Option<String>,
    pub capacity:     Option<i32>,
    pub email:        Option<String>,
    pub contact_form: Option<String>,
    pub age_range:    String,
    pub is_private:   bool,
}

impl TryFrom<EditContactData> for EditedContact {
    type Error = String;

    fn try_from(value: EditContactData) -> Result<Self, Self::Error> {
        let display_name = StringInput::parse(value.display_name);
        let address      = OptionalStringInput::parse(value.address);
        let city         = StringInput::parse(value.city);
        let state        = OptionalStringInput::parse(value.state);
        let zip_code     = OptionalStringInput::parse(value.zip_code);
        let country      = OptionalStringInput::parse(value.country);
        let email        = OptionalStringInput::parse(value.email);
        let contact_form = OptionalStringInput::parse(value.contact_form);
        let age_range    = StringInput::parse(value.age_range);

        Ok(Self {
            contact_id: value.contact_id,
            display_name,
            address,
            city,
            state,
            zip_code,
            country,
            capacity: value.capacity,
            email,
            contact_form,
            age_range,
            is_private: value.is_private
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewContact {
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingContact {
    pub contact_id:   i32,
    pub display_name: String,
    pub address:      Option<String>,
    pub city:         String,
    pub state:        Option<String>,
    pub zip_code:     Option<String>,
    pub country:      Option<String>,
    pub capacity:     Option<i32>,
    pub email:        Option<String>,
    pub contact_form: Option<String>,
    pub age_range:    String,
    pub user_id:      Uuid,
}

#[tracing::instrument(
    name = "Deleting a contact from the database",
    skip(contact_id, pool)
)]
pub async fn delete_contact(
    contact_id: &i32,
    pool:       &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM contacts WHERE contact_id = $1
        "#,
        contact_id
    ).execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Updating contact in database",
    skip(contact, pool)
)]
pub async fn update_contact(
    contact: EditedContact,
    pool:    &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE contacts
        SET 
            display_name = $1,
            address = $2,
            city = $3,
            state = $4,
            zip_code = $5,
            capacity = $6,
            email = $7,
            contact_form = $8,
            age_range = $9,
            is_private = $10
        WHERE contact_id = $11
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
        contact.contact_id
    ).execute(pool)
    .await?;

    Ok(())
}