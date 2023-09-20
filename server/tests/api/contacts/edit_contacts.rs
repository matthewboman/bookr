use byot_server::domain::{ContactResponse};
use crate::helpers::spawn_app;

#[derive(serde::Deserialize, Debug)]
struct Contacts(Vec<ContactResponse>);

#[tokio::test]
async fn unauthenticated_user_cannot_edit_contact() {
    let app     = spawn_app().await;
    let contact = serde_json::json!({
        "contactId":    1, // should error before validating before this would error w/ 400
        "displayName": "try to edit",
        "city":        "asheville",
        "ageRange":    "all",
        "isPrivate":   false
    });
    let response = app.user_edit_contact(contact).await;

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn user_can_edit_contact() {
    // Log in
    let app      = spawn_app().await;
    let response = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Log out
    app.post_logout().await;

    // Admin login and approve contact
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);
    let contact  = app.get_first_pending_contact().await;
    let response = app.approve_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Admin log out
    app.post_logout().await;

    // User login and get contact
    let response = app.test_user_login().await;
    let contact  = app.get_first_contact().await;

    // Edit contact
    let edited = serde_json::json!({
        "contactId":   contact.contact_id,
        "userId":      app.test_user.user_id,
        "displayName": "changed name",
        "city":        "new city",
        "ageRange":    "all",
        "isPrivate":   false,
        "genres":      [1, 2] // add genre 2
    });
    let response = app.user_edit_contact(edited).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn admin_can_edit_contact() {
    // Log in
    let app      = spawn_app().await;
    let response = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Log out
    app.post_logout().await;

    // Admin login and approve contact
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);
    let contact  = app.get_first_pending_contact().await;
    let response = app.approve_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Edit contact
    let contact = app.get_first_contact().await;
    let edited  = serde_json::json!({
        "contactId":   contact.contact_id,
        "userId":      contact.user_id,
        "displayName": "changed name",
        "city":        "new city",
        "ageRange":    "all",
        "isPrivate":   false,
        "genres":      [2] // remove genre 1
    });
    let response = app.admin_edit_contact(edited).await;

    assert_eq!(200, response.status().as_u16());
}