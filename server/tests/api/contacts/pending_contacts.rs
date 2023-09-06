use crate::helpers::spawn_app;

#[tokio::test]
async fn unauthenticated_user_cannot_see_pending_contacts() {
    let app = spawn_app().await;
    let response = app.get_pending_contacts().await;
    
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn non_admin_user_cannot_see_pending_contacts() {
    // Log in
    let app      = spawn_app().await;
    let response = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Get pending contacts
    let response = app.get_pending_contacts().await;
    
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn admin_can_see_pending_contacts() {
    // Log in
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Get pending contacts
    let response = app.get_pending_contacts().await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn admin_approve_pending_contact() {
    // Log in
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Get and approve contact
    let contact  = app.get_first_pending_contact().await;
    let response = app.approve_contact(contact).await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn admin_can_delete_pending_contact() {
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Get contact
    let contact = app.get_first_pending_contact().await;

    // Delete contact
    let json = serde_json::json!({
        "contactId": contact.contact_id,
        "userId":    contact.user_id,
        "address":   contact.address,
        "city":      contact.city,
        "state":     contact.state,
        "zipCode":   contact.zip_code
    });
    let response = app.delete_pending_contact(&json).await;

    assert_eq!(200, response.status().as_u16());
}