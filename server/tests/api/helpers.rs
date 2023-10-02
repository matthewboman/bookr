use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

use byot_server::configuration::{get_configuration, DatabaseSettings, JWTSettings};
use byot_server::domain::{ContactResponse, PendingContact, Review};
use byot_server::email_client::EmailClient;
use byot_server::startup::{Application, get_connection_pool};
use byot_server::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter  = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter,
            std::io::stdout
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter,
            std::io::sink
        );
        init_subscriber(subscriber);
    }
});

#[derive(serde::Deserialize)]
struct Contacts(Vec<ContactResponse>);

#[derive(serde::Deserialize)]
struct PendingContacts(Vec<PendingContact>);

#[derive(serde::Deserialize)]
struct Reviews(Vec<Review>);

pub struct ConfirmationLinks {
    pub html:       reqwest::Url,
    pub plain_text: reqwest::Url,
}

pub struct TestApp {
    pub address:      String,
    pub db_pool:      PgPool,
    pub email_server: MockServer,
    pub port:         u16,
    pub test_user:    TestUser,
    pub admin:        TestUser,
    pub api_client:   reqwest::Client,
    pub email_client: EmailClient,
    pub jwt_settings: JWTSettings,
}

impl TestApp {
    // Shortcuts
    pub async fn admin_login(&self) -> reqwest::Response {
        let login_body = serde_json::json!({
            "email":    &self.admin.email,
            "password": &self.admin.password
        });

        self.post_login(&login_body).await
    }

    pub async fn approve_contact(&self, contact: PendingContact) -> reqwest::Response {
        let json = serde_json::json!({
            "contactId": contact.contact_id,
            "address":   contact.address,
            "city":      contact.city,
            "state":     contact.state,
            "zipCode":   contact.zip_code
        });
        
        self.post_approve_contact(&json).await
    }

    pub async fn create_contact(&self, is_private: bool) -> reqwest::Response {
        let contact  = serde_json::json!({
            "displayName": "test for pending",
            "address":     "123 fake st",
            "city":        "asheville",
            "state":       "NC",
            "zipCode":     "28711",
            "capacity":    100,
            "ageRange":    "all",
            "contactType": "venue",
            "isPrivate":   is_private,
            "genres":      [1]
        });

        self.add_contact(&contact).await
    }

    pub async fn get_first_contact(&self) -> ContactResponse {
        let contacts = self.get_contacts()
            .await
            .json::<Contacts>()
            .await
            .unwrap();
        let contact = contacts.0.first().unwrap();

        // not the most efficient, but * errors with "returns a value referencing data owned by the current function"
        contact.clone()
    }

    pub async fn get_first_pending_contact(&self) -> PendingContact {
        let contacts = self.get_pending_contacts()
            .await
            .json::<PendingContacts>()
            .await
            .unwrap();
        let contact = contacts.0.first().unwrap();

        // not the most efficient, but * errors with "returns a value referencing data owned by the current function"
        contact.clone()
    }

    pub async fn review_contact(&self, contact: ContactResponse) -> reqwest::Response 
    {
        let review = serde_json::json!({
            "contactId": contact.contact_id,
            "title":     "not half bad",
            "body":      "it was all bad",
            "rating":    1
        });

        self.add_review(&review).await
    }

    pub async fn test_user_login(&self) -> reqwest::Response {
        let login_body = serde_json::json!({
            "email":    &self.test_user.email,
            "password": &self.test_user.password
        });

        self.post_login(&login_body).await
    }

    // Routes
    pub async fn add_contact<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/user/add-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn add_review<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/user/review-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn admin_delete_contact<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/admin/delete-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn admin_edit_contact<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/admin/edit-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn admin_edit_review<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/admin/edit-review", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn admin_get_reviews(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/admin/all-reviews", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn admin_delete_review<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/admin/delete-review", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_pending_contact<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/admin/delete-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn generate_reset_token<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/generate-reset-token", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_contacts(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/contacts", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_genres(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/genres", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_reviews(&self, id: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/reviews?contactId={}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_pending_contacts(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/admin/pending-contacts", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_private_contacts(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/user/private-contacts", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_approve_contact<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/admin/approve-pending-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_change_password<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/user/change-password", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/login", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/user/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn reset_password<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/reset-password", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn sign_up<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/signup", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn user_delete_contact<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/user/delete-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn user_delete_review<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/user/delete-review", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn user_edit_contact<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/user/edit-contact", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn user_edit_review<Json>(&self, json: Json) -> reqwest::Response
    where Json: serde::Serialize
    {
        self.api_client
            .post(&format!("{}/user/edit-review", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn user_get_reviews(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/user/my-reviews", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    // Public methods
    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();

            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();

            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();

            confirmation_link
        };

        let html       = get_link(body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(body["TextBody"].as_str().unwrap());
        
        ConfirmationLinks { html, plain_text }
    }

    pub fn get_token_from_links(&self, confirmation_links: ConfirmationLinks) -> String {
        let re = Regex::new(
            r#"=(?<token>.*)"#
        ).expect("failed to compile regex");
    
        let link = confirmation_links.plain_text.into_string();
        let Some(result) = re.captures(&link) else {
            return "".to_string();
        };
        
        result["token"].to_string()
    }
}

pub struct TestUser {
    pub user_id:  Uuid,
    pub email:    String,
    pub password: String
}

impl TestUser {
    pub fn generate() -> Self {
        let email = SafeEmail().fake();

        Self { 
            user_id:  Uuid::new_v4(),
            email,
            password: Uuid::new_v4().to_string()
        }
    }

    pub async fn make_admin(&self, pool: &PgPool) {
        sqlx::query!(
            r#"
            UPDATE users
            SET role = 'admin'
            WHERE user_id = $1
            "#,
            self.user_id
        ).execute(pool)
            .await
            .expect("Failed to update test user to admin");
    }

    pub async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap()
        ).hash_password(self.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        
        sqlx::query!(
            "INSERT INTO users (user_id, email, password_hash) VALUES ($1, $2, $3)",
            self.user_id,
            self.email,
            hash
        ).execute(pool)
            .await
            .expect("Failed to store test user");
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server  = MockServer::start().await;
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port       = 0;
        c.email_client.base_url  = email_server.uri();

        c
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    let port        = application.port();
    let address     = format!("http://127.0.0.1:{}", port);
    let _           = tokio::spawn(application.run_until_stopped());
    let api_client  = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
        port,
        test_user: TestUser::generate(),
        admin: TestUser::generate(),
        api_client,
        email_client: configuration.email_client.client(),
        jwt_settings: configuration.jwt_settings
    };

    test_app.test_user.store(&test_app.db_pool).await;
    test_app.admin.store(&test_app.db_pool).await;
    test_app.admin.make_admin(&test_app.db_pool).await;

    test_app
}