use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::spawn_app;

#[tokio::test]
async fn sign_up_with_correct_info_succeeds() {
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

    let response = app.sign_up(&json).await;

    assert_eq!(response.status().as_u16(), 200);
}