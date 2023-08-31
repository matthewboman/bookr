use crate::helpers::spawn_app;

#[tokio::test]
async fn login_with_correct_info_succeeds() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &app.test_user.password
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_with_bunk_info_fails() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": "hacktheplanet"
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 401);
}