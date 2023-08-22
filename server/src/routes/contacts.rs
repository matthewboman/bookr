use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::ContactResponse;
use crate::error::ContentError;

#[tracing::instrument(
    skip(pool),
)]
pub async fn public_contacts(pool: web::Data<PgPool>) -> Result<HttpResponse, ContentError> {
    let contacts = query_contacts(&pool)
        .await
        .context("Failed to query contacts for guest")?;
     
    Ok(HttpResponse::Ok().json(contacts))   
}

#[tracing::instrument(
    name = "Querying contacts from DB",
    skip(pool)
)]
async fn query_contacts(pool: &PgPool) -> Result<Vec<ContactResponse>, sqlx::Error> {
    let contacts = sqlx::query_as!(
        ContactResponse,
        r#"
        SELECT contact_id, display_name, address, city, state, zip_code, capacity, latitude, longitude, email, contact_form, age_range, country, is_private, user_id
        FROM contacts
        WHERE is_private = false
        AND verified = true
        "#
    ).fetch_all(pool)
    .await?;

    Ok(contacts)
}

// #[tracing::instrument(
//     name = "Querying contacts from DB",
//     skip(pool)
// )]
// async fn query_contacts(
//     pool: &PgPool
// ) -> Result<Vec<ContactResponse>, sqlx::Error> {
//     let contacts = sqlx::query!(
//         r#"
//         SELECT c.contact_id, c.display_name, c.address, c.city, c.state, 
//                c.zip_code, c.capacity, c.latitude, c.longitude, c.email, 
//                c.contact_form, c.age_range, c.country, c.is_private, 
//                c.user_id, r.title, r.body, r.rating, r.review_id
//         FROM contacts c
//         LEFT JOIN reviews r ON c.contact_id = r.contact_id
//         WHERE c.is_private = false
//         AND c.verified = true
//         "#
//     )
//     .fetch_all(pool)
//     .await?
//     .into_iter()
//     .group_by(|row| row.contact_id)
//     .into_iter()
//     .map(|(contact_id, rows)| {
//         let mut contact_reviews = Vec::new();
//         let mut contact_response = None;

//         for row in rows {
//             let review = ContactReview {
//                 review_id: row.review_id,
//                 title: row.title,
//                 body: row.body,
//                 rating: row.rating,
//             };

//             contact_reviews.push(review);

//             if contact_response.is_none() {
//                 contact_response = Some(ContactResponse {
//                     contact_id: contact_id.unwrap_or_default(),
//                     display_name: row.display_name.unwrap_or_default(),
//                     address: row.address,
//                     city: row.city.unwrap_or_default(),
//                     state: row.state,
//                     zip_code: row.zip_code,
//                     capacity: row.capacity,
//                     latitude: row.latitude,
//                     longitude: row.longitude,
//                     email: row.email,
//                     contact_form: row.contact_form,
//                     age_range: row.age_range,
//                     country: row.country,
//                     is_private: row.is_private.unwrap_or_default(),
//                     user_id: row.user_id,
//                     reviews: Vec::new(),
//                 });
//             }
//         }

//         if let Some(mut response) = contact_response {
//             response.reviews = contact_reviews;
//             response
//         } else {
//             unreachable!();
//         }
//     })
//     .collect();

//     Ok(contacts)
// }



// Query and result

// SELECT c.contact_id, c.display_name, c.address, c.city, c.state,
// c.user_id, r.title, r.body, r.rating, r.review_id
// FROM contacts c
// JOIN reviews r ON c.contact_id = r.contact_id
// WHERE c.is_private = false
// AND c.verified = true
// AND c.contact_id = 210;

//  contact_id |     display_name      |      address       |  city   | state | user_id | title |      body       | rating |              review_id               
// ------------+-----------------------+--------------------+---------+-------+---------+-------+-----------------+--------+--------------------------------------
//         210 | El Corazon / Funhouse | 109 Eastlake Ave E | Seattle | WA    |         | bad   | they were dicks |      2 | 3a154140-33aa-43bf-9b2d-c4912373950c
//         210 | El Corazon / Funhouse | 109 Eastlake Ave E | Seattle | WA    |         | ok    | was it fun      |      3 | 2f8f2a7b-7c72-4fde-b748-399fc22a2626