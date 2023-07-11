use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie, SameSite},
    web, HttpResponse
};
use actix_web::error::InternalError;
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::auth::{validate_credentials, AuthError, TokenClaims, Credentials};
use crate::configuration::JWTSettings;
use crate::domain::user::UserLogin;
use crate::utils::error_chain_fmt;

#[tracing::instrument(
    skip(json, pool, jwt_settings),
    fields(
        email=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn login(
    json:         web::Json<UserLogin>,
    pool:         web::Data<PgPool>,
    jwt_settings: web::Data<JWTSettings>,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        email:    json.0.email,
        password: json.0.password.into()
    };

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            println!("validated for {}", user_id);

            let now  = Utc::now();
            let iat  = now.timestamp() as usize;
            let exp  = (now + Duration::minutes(60)).timestamp() as usize;

            let claims: TokenClaims = TokenClaims {
                sub: user_id.to_string(),
                exp,
                iat,
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
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into())
            };

            Err(login_error(e))
        }
    }
}

fn login_error(e: LoginError) -> InternalError<LoginError> {
    let response = HttpResponse::Unauthorized().finish(); // idk if this is the right response

    InternalError::from_response(e, response)
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),

    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}