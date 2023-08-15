use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::{PgPool, postgres::{PgQueryResult}};

use crate::auth::JwtMiddleware;
use crate::domain::PendingContact;
use crate::error::AdminError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonData {
    contact_id: i32
}

#[tracing::instrument(
    skip(req, json, pool),
)]
pub async fn approve_contact(
    req:  HttpRequest,
    json: web::Json<JsonData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AdminError> {
    let ext     = req.extensions();
    let role    = ext.get::<String>().unwrap();
    let admin   = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    // TODO: should this be a transaction as part of the lat/lng script?
    mark_contact_as_verified(json.contact_id, &pool)
        .await
        .context("Failed to delete contact")?;
    
    // TODO: run script to fetch lat/lng and update contact
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, json, pool),
)]
pub async fn delete_pending_contact(
    req:  HttpRequest,
    json: web::Json<JsonData>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AdminError> {
    let ext     = req.extensions();
    let role    = ext.get::<String>().unwrap();
    let admin   = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    admin_delete_contact(json.contact_id, &pool)
        .await
        .context("Failed to delete contact")?;
    
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    skip(req, pool),
)]
pub async fn get_pending_contacts(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> Result<HttpResponse, AdminError> {
    let ext   = req.extensions();
    let role  = ext.get::<String>().unwrap();
    let admin = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    let contacts = query_pending_contacts(&pool)
        .await
        .context("Failed to query pending contacts for admin")?;
    
    Ok(HttpResponse::Ok().json(contacts))
}

// TODO: This might later be shared for functionality to delete contact from map UI
#[tracing::instrument(
    name = "Deleting a contact from the database",
    skip(contact_id, pool)
)]
pub async fn admin_delete_contact(
    contact_id: i32,
    pool:       &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM contacts WHERE contact_id = $1
        "#,
        contact_id
    ).execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Marking a contact as verified in the database",
    skip(contact_id, pool)
)]
pub async fn mark_contact_as_verified(
    contact_id: i32,
    pool:       &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE contacts
        SET verified = true
        WHERE contact_id = $1
        "#,
        contact_id
    ).execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Getting all pending contacts from the database",
    skip(pool)
)]
pub async fn query_pending_contacts(
    pool: &PgPool
) -> Result<Vec<PendingContact>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        PendingContact,
        r#"
        SELECT contact_id, display_name, address, city, state, zip_code, capacity, email, contact_form, age_range, country, user_id
        FROM contacts
        WHERE verified = false
        "#
    ).fetch_all(pool)
    .await?;

    Ok(contacts)
}