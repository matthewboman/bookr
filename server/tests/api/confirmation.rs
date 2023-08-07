use wiremock::{ResponseTemplate, Mock};
use wiremock::matchers::{path, method};

use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!("{}/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_signup_returns_a_200_if_called() {
    let app  = spawn_app().await;
    let json = serde_json::json!({
        "email": "test@test.test",
        "password": "test"
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.sign_up(json).await;

    let email_request      = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    let response = reqwest::get(confirmation_links.html).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_user() {
    let app  = spawn_app().await;
    let json = serde_json::json!({
        "email": "click@click.click",
        "password": "test"
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.sign_up(json).await;

    let email_request      = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT email, status FROM users WHERE email = 'click@click.click'", )
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    
    assert_eq!(saved.email, "click@click.click");
    assert_eq!(saved.status.expect("matches"), "confirmed");
}