use crate::helpers::spawn_app;

#[tokio::test]
async fn unauthenticated_user_cannot_add_contact() {
    let app     = spawn_app().await;
    let contact = serde_json::json!({
        "displayName": "test",
        "city": "asheville"
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn authenticated_user_can_add_contact() {
    // Login
    let app        = spawn_app().await;
    let response   = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let contact  = serde_json::json!({
        "displayName": "test",
        "city": "asheville",
        "ageRange":  "18+",
        "isPrivate": false
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(200, response.status().as_u16());
}