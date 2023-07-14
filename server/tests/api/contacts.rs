use crate::helpers::spawn_app;

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

#[tokio::test]
async fn unauthenticated_user_cannot_add_contact() {
    let app      = spawn_app().await;
    let contact  = serde_json::json!({
        "display_name": "test",
        "city": "asheville"
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn authenticated_user_can_add_contact() {
    let app        = spawn_app().await;
    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &app.test_user.password
    });
    let response   = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let contact  = serde_json::json!({
        "displayName": "test",
        "city": "asheville",
        "isPrivate": false
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(200, response.status().as_u16());
}