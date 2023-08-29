use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    pub age_range:    Option<String>,
    pub user_id:      Option<String>,
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
    pub age_range:      Option<String>,
    pub user_id:        Option<uuid::Uuid>,
    pub is_private:     bool,
    pub average_rating: Option<f32>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContact {
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
    pub age_range:    Option<String>,
    pub user_id:      Option<uuid::Uuid>,
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