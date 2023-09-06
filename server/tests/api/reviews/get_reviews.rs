use crate::helpers::spawn_app;

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