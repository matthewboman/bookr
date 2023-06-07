use crate::helpers::spawn_app;

#[tokio::test]
async fn contacts_returns_a_200() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = client
        .get(&format!("{}/contacts", &app.address))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert_eq!(200, response.status().as_u16());
}