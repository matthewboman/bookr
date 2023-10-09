use byot_server::domain::Review;
use crate::helpers::spawn_app;

#[derive(serde::Deserialize)]
struct Reviews(Vec<Review>);

#[tokio::test]
async fn unauthenticated_user_cannot_edit_review() {
    let app     = spawn_app().await;
    // data won't pass validation but this should fail before that step
    let review = serde_json::json!({
        "contactId": 1,
        "userId":    "1234-qwer",
        "reviewId":  "1234-qwer",
        "title":     "unauthenticated",
        "body":      "this won't work",
        "rating":    1
    });
    let response = app.user_edit_review(review).await;

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn user_can_edit_review() {
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
    let pending  = app.get_first_pending_contact().await;
    let response = app.approve_contact(pending).await;
    assert_eq!(200, response.status().as_u16());

    // Admin log out
    app.post_logout().await;

    // User login and get contact, and review
    let response = app.test_user_login().await;
    let contact  = app.get_first_contact().await;
    let response = app.review_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Get reviews
    let reviews = app.user_get_reviews()
        .await
        .json::<Reviews>()
        .await
        .unwrap();
    let review  = reviews.0.first().unwrap();

    // Edit review
    let json = serde_json::json!({
        "contactId": review.contact_id,
        "userId":    review.user_id,
        "reviewId":  review.review_id,
        "title":     "not half bad",
        "body":      "it was all bad",
        "rating":    1
    });
    let response = app.user_edit_review(&json).await;

    assert_eq!(200, response.status().as_u16());;
}

#[tokio::test]
async fn admin_can_edit_review() {
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

    // User login and get contact, and review
    let response = app.test_user_login().await;
    let contact  = app.get_first_contact().await;
    let response = app.review_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // User log out
    app.post_logout().await;

    // Admin login and edit review
    let response = app.admin_login().await;
    let reviews = app.admin_get_recent_reviews()
        .await
        .json::<Reviews>()
        .await
        .unwrap();
    let review = reviews.0.first().unwrap();
    let json = serde_json::json!({
        "contactId": review.contact_id,
        "userId":    review.user_id,
        "reviewId":  review.review_id,
        "title":     "not half bad",
        "body":      "it was all bad",
        "rating":    1
    });
    let response = app.admin_edit_review(&json).await;

    assert_eq!(200, response.status().as_u16());;
}