use actix_web::{HttpRequest, HttpResponse};
use actix_web::cookie::{time::Duration, Cookie};
use serde_json::json;

use crate::auth::JwtMiddleware;
// use crate::utils::e500;

#[tracing::instrument(
    skip(req),
)]
pub async fn log_out(
    req:  HttpRequest,
    _:    JwtMiddleware
) -> HttpResponse {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(Duration::new(-1, 0))
        .http_only(true)
        .finish();
    
    HttpResponse::Ok()
        .cookie(cookie)
        .finish()
}
