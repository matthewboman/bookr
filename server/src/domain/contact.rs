// use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{Genre, OptionalStringInput, StringInput};

// TODO: use generics to clean up
// https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub average_rating: Option<f32>,
    pub genres:         Vec<Genre>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContactRow {
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
    pub average_rating: Option<f32>,
    pub genre_id:       i32,
    pub genre_name:     String,
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
    pub genres:       Vec<i32>,
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
    pub genres:       Vec<i32>,
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
            is_private: value.is_private,
            genres:     value.genres,
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
    pub genres:       Vec<i32>,
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
    name = "Adding contacts_genres relations",
    skip(contact_id, genres, transaction)
)]
pub async fn add_contact_genre_relation(
    contact_id: &i32, 
    genres:     Vec<i32>, 
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    for genre_id in genres {
        sqlx::query!(
            r#"
            INSERT INTO contacts_genres (contact_id, genre_id)
            VALUES ($1, $2)
            "#,
            contact_id,
            genre_id
        ).execute(&mut *transaction)
        .await?;
    }

    Ok(())
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
    name = "Querying genres for contact",
    skip(contact_id, transaction)
)]
pub async fn query_genres_by_contact(
    contact_id: &i32,
    transaction: &mut Transaction<'_, Postgres>
) -> Result<Vec<i32>, sqlx::Error> {
    let genres = sqlx::query!(
        r#"
        SELECT genre_id FROM contacts_genres
        WHERE contact_id = $1
        "#,
        contact_id
    ).fetch_all(transaction)
    .await?
    .into_iter()
    .map(|r| r.genre_id)
    .collect();

    Ok(genres)
}

#[tracing::instrument(
    name = "Removing contacts_genres relations",
    skip(contact_id, genres, transaction)
)]
async fn remove_contact_genre_relation(
    contact_id:  &i32,
    genres:      Vec<i32>,
    transaction: &mut Transaction<'_, Postgres>
) -> Result<(), sqlx::Error> {
    for genre_id in genres {
        sqlx::query!(
            r#"
            DELETE FROM contacts_genres 
            WHERE contact_id = $1 AND genre_id = $2
            "#,
            contact_id,
            genre_id
        ).execute(&mut *transaction)
        .await?;
    }

    Ok(())
}

#[tracing::instrument(
    name = "Updating contact in database",
    skip(contact, transaction)
)]
pub async fn update_contact(
    contact:     &EditedContact,
    transaction: &mut Transaction<'_, Postgres>
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
    ).execute(transaction)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Updating contact_genres relation in database",
    skip(contact, transaction)
)]
pub async fn update_contact_genres(
    contact:     &EditedContact,
    transaction: &mut Transaction<'_, Postgres>
) -> Result<(), sqlx::Error> {
    let previous_genres  = query_genres_by_contact(&contact.contact_id, transaction)
        .await?;
    let genres_to_add    = contact.genres
        .iter()
        .filter(|g| !previous_genres.contains(g))
        .cloned()
        .collect::<Vec<_>>();
    let genres_to_remove = previous_genres
        .iter()
        .filter(|g| !contact.genres.contains(g))
        .cloned()
        .collect::<Vec<_>>();
    
    if genres_to_add.len() > 0 {
        add_contact_genre_relation(&contact.contact_id, genres_to_add, transaction)
            .await?;
    }

    if genres_to_remove.len() > 0 {
        remove_contact_genre_relation(&contact.contact_id, genres_to_remove, transaction)
            .await?;
    }

    Ok(())
}

pub fn format_contact_response(rows: Vec<ContactRow>) -> Vec<ContactResponse> {
    let mut grouped_contacts: Vec<ContactResponse> = Vec::new();

    for row in rows {
        let genre = Genre {
            genre_id:   row.genre_id,
            genre_name: row.genre_name
        };

        if let Some(existing_contact) = grouped_contacts.iter_mut().find(|c| c.contact_id == row.contact_id) {
            existing_contact.genres.push(genre);
        } else {
            let contact = ContactResponse {
                contact_id:     row.contact_id,
                display_name:   row.display_name,
                address:        row.address,
                city:           row.city,
                state:          row.state,
                zip_code:       row.zip_code,
                country:        row.country,
                capacity:       row.capacity,
                latitude:       row.latitude,
                longitude:      row.longitude,
                email:          row.email,
                contact_form:   row.contact_form,
                age_range:      row.age_range,
                is_private:     row.is_private,
                user_id:        row.user_id,
                average_rating: row.average_rating,
                genres:         [genre].to_vec()
            };
            grouped_contacts.push(contact)
        }
    }
    grouped_contacts
}