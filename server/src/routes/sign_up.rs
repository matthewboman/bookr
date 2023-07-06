use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::{ExposeSecret, Secret};

use sqlx::PgPool;

use serde_json::json;
use uuid::Uuid;


// TODO: make model
#[derive(serde::Deserialize)]
pub struct UserLogin {
    email:    String,
    password: String
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
    user_id:  uuid::Uuid,
    email:    String,
    password_hash: String
}

#[tracing::instrument(
    skip(json, pool),
    fields(
        email=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn sign_up(
    json:    web::Json<UserLogin>,
    pool:    web::Data<PgPool>
) -> impl Responder {
    // TODO: check if user exists

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(json.password.as_bytes(), &salt)
        .expect("Error hashing password")
        .to_string();
    let user_id = Uuid::new_v4();
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO USERS (user_id, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        user_id,
        json.email.to_string(),
        hashed_password
    ).fetch_one(&**pool).await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "user": &user
                })
            });
            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}