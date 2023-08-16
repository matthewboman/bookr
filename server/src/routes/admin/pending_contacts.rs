use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::auth::JwtMiddleware;
use crate::domain::PendingContact;
use crate::error::AdminError;
use crate::gmaps_api_client::{get_latlng_from_address, GoogleMapsAPIClient, Location};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonData {
    contact_id: i32,
    address:    String,
    city:       String,
    state:      String,
    zip_code:   String,
}

#[derive(serde::Serialize)]
struct JsonResponse {
    message: String,
}

#[tracing::instrument(
    skip(req, json, pool, g_client),
)]
pub async fn approve_contact(
    req:      HttpRequest,
    json:     web::Json<JsonData>,
    pool:     web::Data<PgPool>,
    g_client: web::Data<GoogleMapsAPIClient>,
    _:        JwtMiddleware,
) -> Result<HttpResponse, AdminError> {
    let ext     = req.extensions();
    let role    = ext.get::<String>().unwrap();
    let admin   = String::from("admin");

    if role.to_string() != admin {
        return Err(AdminError::InvalidToken)
    }

    // TODO: Would it be better to only send `contact_id` from API and return the Contact here?
    // Or allow admin to edit address, updating in database here or on successful Google API response?
    mark_contact_as_verified(&json.contact_id, &pool)
        .await
        .context("Failed to delete contact")?;

    let address = format!(
        "{} {}, {} {}",
        &json.address,
        &json.city,
        &json.state,
        &json.zip_code,
    );
    let mut msg = format!("Location of {} successfully added", &address);

    // Don't fail if location can't be found from address.
    // Instead, let admin know there was an error.
    match get_latlng_from_address(&g_client, &address).await {
        Ok(lat_lng) => {
            add_lat_lng_to_contact(&json.contact_id, lat_lng, &pool)
                .await
                .context("Failed to update contact lat/lng")?;
        }
        Err(e) => {
            msg = format!("{}", e);
        }
    }

    let json_response = JsonResponse {
        message: msg.to_string()
    };
    
    Ok(
        HttpResponse::Ok()
            .json(json_response)
    )
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

    admin_delete_contact(&json.contact_id, &pool)
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
    contact_id: &i32,
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
    contact_id: &i32,
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

#[tracing::instrument(
    name = "Updating contact latitude and longitude in the database",
    skip(contact_id, pool, lat_lng)
)]
pub async fn add_lat_lng_to_contact(
    contact_id: &i32,
    lat_lng:    Location,
    pool:       &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE contacts
        SET latitude = $1, longitude = $2
        WHERE contact_id = $3
        "#,
        lat_lng.lat,
        lat_lng.lng,
        contact_id
    ).execute(pool)
    .await?;

    Ok(())
}