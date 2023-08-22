use byot_server::domain::{PendingContact, Review};
use crate::helpers::spawn_app;

#[derive(serde::Deserialize)]
struct PendingContacts(Vec<PendingContact>);

#[derive(serde::Deserialize)]
struct Reviews(Vec<Review>);

#[tokio::test]
async fn reviews_returns_a_400_if_no_contact_id() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = app.get_reviews("").await;

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn reviews_returns_a_200() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = app.get_reviews("1").await;

    assert_eq!(200, response.status().as_u16());
}

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
    let app        = spawn_app().await;
    let login_body = serde_json::json!({
        "email":    &app.test_user.email,
        "password": &app.test_user.password
    });
    let response   = app.post_login(&login_body).await;

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
    let app        = spawn_app().await;
    // Note: using admin user and pending contacts to bypass approval step
    app.test_user.make_admin(&app.db_pool).await;
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

    // Review contact
    let review = serde_json::json!({
        "contactId": contact.contact_id,
        "title":     "not half bad",
        "body":      "it was all bad",
        "rating":    1
    });
    let response = app.add_review(&review).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn admin_can_delete_review() {
    // Log in
    let app        = spawn_app().await;
    // Note: using admin user and pending contacts to bypass approval step
    app.test_user.make_admin(&app.db_pool).await;
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

    // Review contact
    let review = serde_json::json!({
        "contactId": contact.contact_id,
        "title":     "not half bad",
        "body":      "it was all bad",
        "rating":    1
    });
    let response = app.add_review(&review).await;

    assert_eq!(200, response.status().as_u16());

    // Get reviews
    let reviews = app.admin_get_reviews()
        .await
        .json::<Reviews>()
        .await
        .unwrap();
    let review  = reviews.0.first().unwrap();

    // Delete review
    let json = serde_json::json!({
        "reviewId": review.review_id
    });
    let response = app.admin_delete_review(&json).await;

    assert_eq!(200, response.status().as_u16());
}

// user can delete own review

// user cannot delete another's review