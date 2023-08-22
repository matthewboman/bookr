use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Review {
    pub review_id:  Uuid,
    pub contact_id: i32,
    pub user_id:    Uuid,
    pub title:      Option<String>,
    pub body:       Option<String>,
    pub rating:     i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResponse {
    pub review_id:  Uuid,
    pub title:      Option<String>,
    pub body:       Option<String>,
    pub rating:     i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactReview {
    pub contact_id: i32,
    pub rating:     f32,
    pub reviews:    Vec<ReviewResponse>
}