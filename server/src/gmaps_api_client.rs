use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use urlencoding::encode;

pub async fn get_latlng_from_address(
    gmaps_api_client: &GoogleMapsAPIClient,
    address:          &str
) -> Result<Location, Box<dyn std::error::Error>> {
    let encoded = encode(&address);
    let url     = format!(
        "{}?address={}&key={}",
        gmaps_api_client.api_url,
        encoded,
        gmaps_api_client.api_key.expose_secret()
    );
    let response: ApiResponse = reqwest::get(url)
        .await?
        .json()
        .await?;
    
    if let Some(result) = response.results.first() {
        Ok(result.geometry.location)
    } else {
        let e = format!("No results found for {}", &address);
        Err(e.into())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GoogleMapsAPIClient {
    api_url: String,
    api_key: Secret<String>,
}

impl GoogleMapsAPIClient {
    pub fn new(
        api_url: String,
        api_key: Secret<String>,
    ) -> GoogleMapsAPIClient {
        Self {
            api_url,
            api_key
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<ResultObject>,
    status:  String, // TODO: Should I use this? I could return status to admin instead of "No results found"
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ResultObject {
    address_components: Vec<AddressComponent>,
    formatted_address:  String,
    geometry:           Geometry,
    place_id:           String,
    types:              Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AddressComponent {
    long_name:  String,
    short_name: String,
    types:      Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Geometry {
    bounds:        Bounds,
    location:      Location,
    location_type: String,
    viewport:      Bounds,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Bounds {
    northeast: Location,
    southwest: Location,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Location {
    pub lat: f32,
    pub lng: f32,
}