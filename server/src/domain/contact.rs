use serde::{Deserialize, Serialize};

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
pub struct ContactResponse {
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
    pub user_id:      Option<uuid::Uuid>,
    pub is_private:   bool
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NewContact {
//     pub display_name: String,
//     pub address:      Option<String>,
//     pub city:         String,
//     pub state:        Option<String>,
//     pub zip_code:     Option<String>,
//     pub capacity:     Option<i32>,
//     pub email:        Option<String>,
//     pub contact_form: Option<String>,
//     pub age_range:    Option<String>,
//     pub is_private:   bool,
//     pub user_id:      Option<uuid::Uuid>,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContact {
    pub display_name: String,
    pub address:      String,
    pub city:         String,
    pub state:        String,
    pub zip_code:     String,
    pub capacity:     i32,
    pub email:        String,
    pub contact_form: String,
    pub age_range:    String,
    pub is_private:   bool,
}