use byot_server::domain::{PendingContact};
use crate::helpers::spawn_app;

#[derive(serde::Deserialize)]
struct PendingContacts(Vec<PendingContact>);

impl std::fmt::Debug for PendingContacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)
    }
}

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
async fn admin_approve_pending_contact() {
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
        "displayName": "test for pending",
        "city": "asheville",
        "state": "NC",
        "zipCode": "28711",
        "capacity": 100,
        "ageRange": "allAges",
        "isPrivate": false
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(200, response.status().as_u16());

    // Get contact
    let contacts = app.get_pending_contacts()
        .await
        .json::<PendingContacts>()
        .await
        .unwrap();
    let contact = contacts.0.first().unwrap();

    // Approve contact
    let json = serde_json::json!({
        "contactId": contact.contact_id
    });
    let response = app.approve_contact(&json).await;

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
        "displayName": "test for pending",
        "city": "asheville",
        "state": "NC",
        "zipCode": "28711",
        "capacity": 100,
        "ageRange": "allAges",
        "isPrivate": false
    });
    let response = app.add_contact(&contact).await;
    
    assert_eq!(200, response.status().as_u16());

    // Get contact
    let contacts = app.get_pending_contacts()
        .await
        .json::<PendingContacts>()
        .await
        .unwrap();
    let contact = contacts.0.first().unwrap();

    // Delete contact
    let json = serde_json::json!({
        "contactId": contact.contact_id
    });
    let response = app.delete_pending_contact(&json).await;

    assert_eq!(200, response.status().as_u16());
}