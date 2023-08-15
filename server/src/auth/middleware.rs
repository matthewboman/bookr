use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    http,
    web,
    Error as ActixWebError, 
    FromRequest, 
    HttpMessage, 
    HttpRequest,
};
use core::fmt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::configuration::JWTSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub:  String,
    pub iat:  usize,
    pub exp:  usize,
    pub role: String,
}

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
    pub role:    String,
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

impl FromRequest for JwtMiddleware { 
    type Error  = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_settings = req.app_data::<web::Data<JWTSettings>>().unwrap();
        let jwt_secret   = jwt_settings.secret.expose_secret();
        
        let token = req.cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                // TODO: app isn't getting authorization header from frontend...
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
        let role    = claims.role.to_string();

        req.extensions_mut().insert::<uuid::Uuid>(user_id.to_owned());
        req.extensions_mut().insert::<String>(role.clone());

        ready(
            Ok(JwtMiddleware { user_id, role })
        )
    }
}