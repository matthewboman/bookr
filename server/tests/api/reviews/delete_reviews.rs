use byot_server::domain::Review;
use crate::helpers::spawn_app;

#[derive(serde::Deserialize)]
struct Reviews(Vec<Review>);

#[tokio::test]
async fn admin_can_delete_review() {
    // Log in
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Get and review contact
    let contact  = app.get_first_pending_contact().await;
    let response = app.review_contact(contact).await;
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
        "reviewId": review.review_id,
        "userId":   review.user_id
    });
    let response = app.admin_delete_review(&json).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn user_can_delete_own_review() {
    // Log in
    let app      = spawn_app().await;
    let response = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Admin login
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Admin approve contact
    let contact  = app.get_first_pending_contact().await;
    let response = app.approve_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Get and review contact
    let contact  = app.get_first_pending_contact().await;
    let response = app.review_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Get reviews
    let reviews = app.user_get_reviews()
        .await
        .json::<Reviews>()
        .await
        .unwrap();
    let review  = reviews.0.first().unwrap();

    // Delete review
    let json = serde_json::json!({
        "reviewId": review.review_id,
        "userId":   app.test_user.user_id
    });
    let response = app.admin_delete_review(&json).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn user_cannot_delete_another_users_review() {
    // Admin login
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Admin approve contact
    let contact  = app.get_first_pending_contact().await;
    let response = app.approve_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Admin get and review contact
    let contact  = app.get_first_pending_contact().await;
    let response = app.review_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Get reviews
    let reviews = app.user_get_reviews()
        .await
        .json::<Reviews>()
        .await
        .unwrap();
    let review  = reviews.0.first().unwrap();

    // User login
    let response = app.test_user_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Attempt to delete review
    let json = serde_json::json!({
        "reviewId": review.review_id,
        "userId":   app.test_user.user_id
    });
    let response = app.admin_delete_review(&json).await;

    assert_eq!(401, response.status().as_u16());
}