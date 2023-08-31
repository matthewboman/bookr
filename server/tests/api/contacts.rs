use byot_server::domain::{ContactResponse};
use crate::helpers::spawn_app;

#[derive(serde::Deserialize, Debug)]
struct Contacts(Vec<ContactResponse>);

#[tokio::test]
async fn contacts_returns_a_200() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = app.get_contacts().await;
    
    assert_eq!(200, response.status().as_u16());
}

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
        "isPrivate": false
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn private_contacts_returns_401_for_unauthenticated_user() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = app.get_private_contacts().await;

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn private_contacts_returns_200_for_authenticated_user() {
    // Login
    let app        = spawn_app().await;
    let response   = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Get private contacts
    let response = app.get_private_contacts().await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn private_contact_not_returned_with_public_contacts() {
    // Login
    let app        = spawn_app().await;
    let response   = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create private contact
    let is_private = true;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Query public contacts
    let response: Contacts = app.get_contacts()
        .await
        .json()
        .await
        .unwrap();

    assert!(response.0.is_empty());
}