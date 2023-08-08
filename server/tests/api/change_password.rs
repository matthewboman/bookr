use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn must_be_logged_in_to_change_password() {
    let app = spawn_app().await;
    
    let new_password = Uuid::new_v4().to_string();
    let response     = app.post_change_password(&serde_json::json!({
        "currentPassword":  Uuid::new_v4().to_string(),
        "newPassword":      &new_password,
        "newPasswordCheck": &new_password
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
        "currentPassword":  &wrong_password,
        "newPassword":      &new_password,
        "newPasswordCheck": &new_password
    }))
    .await;

    assert_eq!(response.status().as_u16(), 401); // Returns 400
}

#[tokio::test]
async fn changing_password_works() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Login with current password
    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &app.test_user.password
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    // Change password
    let response = app.post_change_password(&serde_json::json!({
        "currentPassword":  &app.test_user.password,
        "newPassword":      &new_password,
        "newPasswordCheck": &new_password
    }))
    .await;

    assert_eq!(response.status().as_u16(), 200);

    // Log out
    let response = app.post_logout().await;

    // Login with new password
    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &new_password
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200); // Returns 400
}