use serde::{Deserialize, Serialize};
use secrecy::Secret;

#[derive(Debug, Deserialize, Serialize)]
pub struct CleanUser {
    pub user_id: uuid::Uuid,
    pub email:   String
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub user_id:       uuid::Uuid,
    pub email:         String,
    pub password_hash: Secret<String>
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email:    String,
    pub password: Secret<String>
}