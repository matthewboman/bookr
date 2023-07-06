use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::auth::JwtMiddleware;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewContact {
    display_name: String,
    address:      Option<String>,
    city:         String,
    state:        Option<String>,
    zip_code:     Option<String>,
    country:      Option<String>,
    capacity:     Option<i32>,
    email:        Option<String>,
    contact_form: Option<String>,
    age_range:    Option<String>,
}

pub async fn add_contact(
    req:  HttpRequest,
    json: web::Json<NewContact>,
    pool: web::Data<PgPool>,
    _:    JwtMiddleware
) -> HttpResponse {
    let ext     = req.extensions();
    // let user_id = ext.get::<uuid::Uuid>().unwrap();

    println!("{}", json.display_name);
    // println!("added by {}", user_id);

    HttpResponse::Ok().finish()
}