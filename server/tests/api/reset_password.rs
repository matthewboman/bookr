use wiremock::{ResponseTemplate, Mock};
use wiremock::matchers::{path, method};

use crate::helpers::spawn_app;

#[tokio::test]
async fn resetting_password_returns_401_if_no_user_exists() {
    let app  = spawn_app().await;
    let body = serde_json::json!({
        "email": "nope@cant.reset"
    });

    let response = app.generate_reset_token(body).await;
    
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn resetting_password_returns_a_200_if_email_exists() {
    let app  = spawn_app().await;
    let body = serde_json::json!({
        "email": &app.test_user.email
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.generate_reset_token(body).await;
    
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn password_reset_link_returns_401_if_no_token_exists() {
    let app  = spawn_app().await;
    let body = serde_json::json!({
        "reset_token":        "YOLO",
        "new_password":       "password",
        "new_password_check": "password"
    });

    let response = app.reset_password(body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn password_reset_link_returns_200_for_correct_token() {
    let app  = spawn_app().await;
    let body = serde_json::json!({
        "email": &app.test_user.email
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.generate_reset_token(body).await;

    let email_request      = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);
    let reset_token        = app.get_token_from_links(confirmation_links);

    let body = serde_json::json!({
        "reset_token":        reset_token,
        "new_password":       "password",
        "new_password_check": "password"
    });

    let response = app.reset_password(body).await;

    assert_eq!(response.status().as_u16(), 200);
}