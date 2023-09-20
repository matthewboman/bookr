use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::Context;
use itertools::Itertools;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::JwtMiddleware;
use crate::domain::{ContactResponse, Genre};
use crate::error::ContentError;

#[tracing::instrument(
    skip(req, pool)
)]
pub async fn private_contacts(
    req:  HttpRequest,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware,
) -> Result<HttpResponse, ContentError> {
    let ext      = req.extensions();
    let user_id  = ext.get::<uuid::Uuid>().unwrap();
    let verified = true; // TODO: user's might want a list of their pending contacts. Created `pending_private_contacts` when building edit workflow.
 
    let contacts = query_private_contacts(user_id, verified, &pool)
        .await
        .context("Failed to get private contacts from database")?;
    
    Ok(HttpResponse::Ok().json(contacts))    
}

#[tracing::instrument(
    name = "Querying private contacts from DB",
    skip(pool, user_id, verified)
)]
async fn query_private_contacts(
    user_id:  &Uuid,
    verified: bool,
    pool:     &PgPool
) -> Result<Vec<ContactResponse>, sqlx::Error> {
    let contacts: Vec<ContactResponse> = sqlx::query!(
        r#"
        SELECT c.contact_id, c.display_name, c.address, c.city, c.state, 
               c.zip_code, c.capacity, c.latitude, c.longitude, c.email, 
               c.contact_form, c.age_range, c.country, c.is_private, c.user_id,
               ROUND(AVG(r.rating), 2)::real AS average_rating,
               g.genre_name, g.genre_id
        FROM contacts c
        LEFT JOIN reviews r ON c.contact_id = r.contact_id
        LEFT JOIN contacts_genres ON c.contact_id = contacts_genres.contact_id
        LEFT JOIN genres g on g.genre_id = contacts_genres.genre_id
        WHERE c.is_private = true
        AND c.verified = $1
        AND c.user_id = $2
        GROUP BY c.contact_id, g.genre_name, g.genre_id
        "#,
        verified,
        user_id
    ).fetch_all(pool)
    .await?
    .into_iter()
    .group_by(|row| row.contact_id)
    .into_iter()
    // TODO: extract into method.
    // `format_contact_response` in `domain/contacts.rs` is an attempt that runs into type issues
    .map(|(contact_id, rows) | {
        let mut contact_genres   = Vec::new();
        let mut contact_response = None;
    
        for row in rows {
            if contact_response.is_none() {
                contact_response = Some(ContactResponse {
                    contact_id,
                    display_name:   row.display_name,
                    address:        row.address,
                    city:           row.city,
                    state:          row.state,
                    zip_code:       row.zip_code,
                    capacity:       row.capacity,
                    latitude:       row.latitude,
                    longitude:      row.longitude,
                    email:          row.email,
                    contact_form:   row.contact_form,
                    age_range:      row.age_range,
                    country:        row.country,
                    is_private:     row.is_private,
                    user_id:        row.user_id,
                    average_rating: row.average_rating,
                    genres:         Vec::new()
                });
            }
    
            let genre = Genre {
                genre_id:   row.genre_id,
                genre_name: row.genre_name
            };
    
            contact_genres.push(genre);
        }
    
        if let Some(mut response) = contact_response {
            response.genres = contact_genres;
            response       
        } else {
            unreachable!();
        }
    })
    .collect();

    Ok(contacts) 
}