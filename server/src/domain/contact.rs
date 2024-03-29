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
    pub contact_type: Option<String>,
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
    pub contact_type:   Option<String>,
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
    pub contact_type:   Option<String>,
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
    pub contact_type: String,
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
    pub contact_type: String,
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
        let contact_type = StringInput::parse(value.contact_type);

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
            contact_type,
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
    pub contact_type: String,
    pub genres:       Vec<i32>,
}

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
    pub contact_type: Option<String>,
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

#[tracing::instrument(
    name = "Querying contacts from DB",
    skip(pool, contact_id)
)]
pub async fn query_contact_by_id(
    pool:       &PgPool,
    contact_id: &i32,
) -> Result<Option<ContactResponse>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        ContactRow,
        r#"
        SELECT c.contact_id, c.display_name, c.address, c.city, c.state, c.zip_code, 
               c.capacity, c.latitude, c.longitude, c.email, c.contact_form, 
               c.age_range, c.country, c.is_private, c.user_id, c.contact_type,
               ROUND(AVG(r.rating), 2)::real AS average_rating,
               g.genre_name, g.genre_id
        FROM contacts c
        LEFT JOIN reviews r ON c.contact_id = r.contact_id
        LEFT JOIN contacts_genres ON c.contact_id = contacts_genres.contact_id
        LEFT JOIN genres g on g.genre_id = contacts_genres.genre_id
        WHERE c.contact_id = $1
        GROUP BY c.contact_id, g.genre_name, g.genre_id
        "#,
        contact_id
    ).fetch_all(pool)
    .await?;

    let formatted = format_contact_response(contacts);
    let contact   = formatted.first();

    Ok(contact.cloned())
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
            is_private = $10,
            contact_type = $11
        WHERE contact_id = $12
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
                contact_type:   row.contact_type,
                genres:         [genre].to_vec()
            };
            grouped_contacts.push(contact)
        }
    }
    grouped_contacts
}