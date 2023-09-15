use crate::helpers::spawn_app;

#[tokio::test]
async fn genres_returns_a_200() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = app.get_genres().await;
    
    assert_eq!(200, response.status().as_u16());
}