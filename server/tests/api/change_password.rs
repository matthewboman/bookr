use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn must_be_logged_in_to_change_password() {
    let app = spawn_app().await;
    
    let new_password = Uuid::new_v4().to_string();
    let response     = app.post_change_password(&serde_json::json!({
        "current_password":   Uuid::new_v4().to_string(),
        "new_password":       &new_password,
        "new_password_check": &new_password
    })).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = spawn_app().await;

    let new_password   = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "email": &app.test_user.email,
        "password": &app.test_user.password
    })).await;

    let response = app.post_change_password(&serde_json::json!({
        "current_password":   &wrong_password,
        "new_password":       &new_password,
        "new_password_check": &new_password
    }))
    .await;

    assert_eq!(response.status().as_u16(), 401);
}