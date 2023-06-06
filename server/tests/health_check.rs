use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use byot_server::configuration::{get_configuration, DatabaseSettings};
use byot_server::startup::run;
use byot_server::telemetry::{get_subscriber, init_subscriber};



#[tokio::test]
async fn health_check_works() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert!(response.status().is_success());
}

#[tokio::test]
async fn contacts_returns_a_200() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = client
        .get(&format!("{}/contacts", &app.address))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert_eq!(200, response.status().as_u16());
}