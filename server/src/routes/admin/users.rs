use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::error::AdminError;
use crate::utils::is_admin;

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn get_all_users(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AdminError> {
    is_admin(req)?;

    // TODO: idk if we need this
}