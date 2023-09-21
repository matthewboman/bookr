use crate::helpers::spawn_app;

#[tokio::test]
async fn genres_returns_a_200() {
    let app      = spawn_app().await;
    let client   = reqwest::Client::new();
    let response = app.get_genres().await;
    
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn adding_genre_to_contact_works() {
    // Log in
    let app      = spawn_app().await;
    let response = app.admin_login().await;
    assert_eq!(response.status().as_u16(), 200);

    // Create contact
    let is_private = false;
    let response   = app.create_contact(is_private).await;
    assert_eq!(200, response.status().as_u16());

    // Approve contact
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
        "genres":      [1, 2]
    });
    let response = app.admin_edit_contact(edited).await;
    assert_eq!(200, response.status().as_u16());

    // Compare updated genre list
    let contact        = app.get_first_contact().await;
    let genres: Vec<_> = contact.genres.into_iter().map(|g| g.genre_id).collect();

    assert_eq!(genres, [1, 2]);
}

#[tokio::test]
async fn changing_genre_list_works() {
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

    // Admin log in and approve contact
    let response = app.admin_login().await;
    let contact  = app.get_first_pending_contact().await;
    let response = app.approve_contact(contact).await;
    assert_eq!(200, response.status().as_u16());

    // Log out
    app.post_logout().await;

    // User log in and edit contact
    let response = app.test_user_login().await;
    let contact = app.get_first_contact().await;
    let edited  = serde_json::json!({
        "contactId":   contact.contact_id,
        "userId":      contact.user_id,
        "displayName": "changed name",
        "city":        "new city",
        "ageRange":    "all",
        "isPrivate":   false,
        "genres":      [2]
    });
    let response = app.user_edit_contact(edited).await;
    assert_eq!(200, response.status().as_u16());

    // Compare updated genre list
    let contact        = app.get_first_contact().await;
    let genres: Vec<_> = contact.genres.into_iter().map(|g| g.genre_id).collect();

    assert_eq!(genres, [2]);
}