use actix_web::{web, HttpResponse};
use actix_web::error::InternalError;
use secrecy::Secret;
use sqlx::PgPool;

use crate::auth::{validate_credentials, AuthError, Credentials};
use crate::utils::error_chain_fmt;
use crate::session_state::TypedSession;

#[derive(serde::Deserialize)]
pub struct UserLogin {
    email:    String,
    password: Secret<String>
}

#[tracing::instrument(
    skip(json, pool, session),
    fields(
        email=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn login(
    json:    web::Json<UserLogin>,
    pool:    web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        email:    json.0.email,
        password: json.0.password
    };

    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("uesr_id", &tracing::field::display(&user_id));

            session.renew();
            session.insert_user_id(user_id)
                .map_err(|e| login_error(LoginError::UnexpectedError(e.into())))?;
            
            Ok(HttpResponse::Ok().finish())
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