use actix_web::{web, HttpResponse};
use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::domain::StringInput;
use crate::error::GeocodingError;
use crate::gmaps_api_client::{get_latlng_from_address, GoogleMapsAPIClient, Location};
use crate::redis_cli::{get_data_as_json, store_data_as_json};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CityData {
    city: String
}

#[tracing::instrument(
    skip(json, redis, g_client)
)]
pub async fn find_coordinates_for_city(
    json:     web::Json<CityData>,
    redis:    web::Data<redis::Client>,
    g_client: web::Data<GoogleMapsAPIClient>,
) -> Result<HttpResponse, GeocodingError> {
    let city = StringInput::parse(json.city.clone());
    let city = city.to_lowercase();
    let mut conn = redis.get_tokio_connection()
        .await
        .context("Could not reg async Redis connection")?;
    let cached_location: Option<Location> = get_data_as_json(&mut conn, &city)
        .await
        .context("Could not get cached city from Redis")?;

    if let Some(cached_location) = cached_location {
        return Ok(HttpResponse::Ok().json(cached_location))
    }

    let lat_lng = get_latlng_from_address(&g_client, &city)
        .await
        .context("Could not get coordinates from Google API")?;

    store_data_as_json(&mut conn, &city, &lat_lng)
        .await
        .context("Could not store city in Redis")?;

    Ok(HttpResponse::Ok().json(lat_lng))
}