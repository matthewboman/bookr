use actix_web::{
    http,
    web,
    Error as ActixWebError, 
    FromRequest, 
    HttpMessage, 
    HttpRequest,
    HttpResponse
};
use actix_web::body::MessageBody;
use actix_web::dev::{Payload, ServiceRequest, ServiceResponse};
use actix_web::error::{ErrorUnauthorized, InternalError};
use actix_web_lab::middleware::Next;
use core::fmt;

use jsonwebtoken::{decode, DecodingKey, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use std::ops::Deref;
use uuid::Uuid;

use crate::session_state::TypedSession;
use crate::utils::e500;
use crate::configuration::JWTSettings;

use actix_web::http::header::{self, HeaderMap, HeaderValue};

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status:  String,
    message: String,
}
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

// TODO: update to be a service request
impl FromRequest for JwtMiddleware { 
    type Error  = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_settings = req.app_data::<web::Data<JWTSettings>>().unwrap();
        let jwt_secret   = jwt_settings.secret.expose_secret();

        println!("secret {}", jwt_secret.clone());
        
        let token = req.cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });
        
        if token.is_none() {
            let json_error = ErrorResponse {
                status:  "fail".to_string(),
                message: "You are not logged in".to_string()
            };
            return ready(
                Err(ErrorUnauthorized(json_error))
            );
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default()
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status:  "fail".to_string(),
                    message: "Invalid token".to_string()
                };
                return ready(
                    Err(ErrorUnauthorized(json_error))
                );
            }
        };

        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();

        req.extensions_mut().insert::<uuid::Uuid>(user_id.to_owned());

        ready(
            Ok(JwtMiddleware { user_id })
        )
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next:    Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let headers = req.headers();
        let keys = &headers.keys().cloned().collect::<Vec<_>>();

        // println!("has cookie: {}", &keys.contains(&header::COOKIE));

        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(user_id) => {
            println!("user_id: {}", user_id);
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        },
        None => {
            println!("none");
            let response = HttpResponse::Unauthorized().finish();
            let e = anyhow::anyhow!("The user has not logged in");

            Err(InternalError::from_response(e, response).into())
        }
    }
}