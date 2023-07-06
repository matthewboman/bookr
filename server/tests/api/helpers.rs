use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

use byot_server::configuration::{get_configuration, DatabaseSettings, JWTSettings};
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
    pub api_client:   reqwest::Client,
    pub email_client: EmailClient,
    pub jwt_settings: JWTSettings,
}

impl TestApp {
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
}

pub struct TestUser {
    pub user_id:  Uuid,
    pub email:    String,
    pub password: String
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id:  Uuid::new_v4(),
            email:    Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string()
        }
    }

    pub async fn login(&self, app: &TestApp) {
        app.post_login(&serde_json::json!({
            "email":    &self.email,
            "password": &self.password,
        }))
        .await;
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
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();

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
        api_client,
        email_client: configuration.email_client.client(),
        jwt_settings: configuration.jwt_settings
    };

    test_app.test_user.store(&test_app.db_pool).await;

    test_app
}