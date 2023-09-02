use crate::helpers::spawn_app;

#[tokio::test]
async fn unauthenticated_user_cannot_add_review() {
    let app    = spawn_app().await;
    let review = serde_json::json!({
        "contactId": 1,
        "title":     "not half bad",
        "body":      "it was all bad",
        "rating":    1
    });
    let response = app.add_review(&review).await;
    
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn review_must_have_valid_contact_id() {
    // Log in
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Attempt to review non-existant contact
    let review = serde_json::json!({
        "contactId": 100, // will succeed if `1`
        "title":     "not half bad",
        "body":      "it was all bad",
        "rating":    1
    });
    let response = app.add_review(&review).await;

    assert_eq!(500, response.status().as_u16());
}

#[tokio::test]
async fn authenticated_user_can_review_contact() {
    // Log in
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Approve contact
    let pending  = app.get_first_pending_contact().await;
    let response = app.approve_contact(pending).await;
    assert_eq!(200, response.status().as_u16());

    // Get and review contact
    let contact  = app.get_first_contact().await;
    let response = app.review_contact(contact).await;
    assert_eq!(200, response.status().as_u16());
}