use crate::helpers::spawn_app;

#[tokio::test]
async fn unauthenticated_user_cannot_see_pending_contacts() {
    let app = spawn_app().await;
    let response = app.get_pending_contacts().await;
    
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn non_admin_user_cannot_see_pending_contacts() {
    let app        = spawn_app().await;
    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &app.test_user.password
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app.get_pending_contacts().await;
    
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn admin_can_see_pending_contacts() {
    let app        = spawn_app().await;

    app.test_user.make_admin(&app.db_pool).await;

    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &app.test_user.password
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app.get_pending_contacts().await;
    
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn admin_can_delete_pending_contact() {
    // Set up
    let app = spawn_app().await;
    app.test_user.make_admin(&app.db_pool).await;

    // Log in
    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &app.test_user.password
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let contact  = serde_json::json!({
        "displayName": "test",
        "city": "asheville",
        "isPrivate": false
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(200, response.status().as_u16());

    // Get contact
    // let contacts = response

    // Delete contact
}