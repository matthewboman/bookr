use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie, SameSite},
    web, HttpResponse
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::auth::{validate_credentials, TokenClaims, Credentials};
use crate::configuration::JWTSettings;
use crate::domain::user::UserLogin;
use crate::error::LoginError;

#[tracing::instrument(
    skip(json, pool, jwt_settings),
    fields(
        email=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn log_in(
    json:         web::Json<UserLogin>,
    pool:         web::Data<PgPool>,
    jwt_settings: web::Data<JWTSettings>,
) -> Result<HttpResponse, LoginError> { 
    let credentials = Credentials {
        email:    json.0.email,
        password: json.0.password.into()
    };

    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));

    let user = validate_credentials(credentials, &pool).await?;
    let now  = Utc::now();
    let iat  = now.timestamp() as usize;
    let week = Duration::weeks(1);
    let exp  = (now + week).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user.user_id.to_string(),
        exp,
        iat,
        role: user.role.to_string()
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_settings.secret.expose_secret().as_ref())
    ).unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .same_site(SameSite::None) // TODO: is this safe?
        .finish();

    Ok(
        HttpResponse::Ok()
            .cookie(cookie)
            .json(json!({
                "status": "success",
                "token": token
            }))
    ) 
}