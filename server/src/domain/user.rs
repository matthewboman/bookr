use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CleanUser {
    pub user_id: uuid::Uuid,
    pub email:   String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id:       uuid::Uuid,
    pub email:         String,
    pub password_hash: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub email:    String,
    pub password: String
}