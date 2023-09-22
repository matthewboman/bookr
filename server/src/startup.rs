use actix_cors::Cors;
use actix_web::{
    web,
    cookie::Key,
    dev::Server,
    http::header, 
    App, 
    HttpServer
};
use secrecy::{ExposeSecret, Secret};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::configuration::{DatabaseSettings, JWTSettings, Settings};
use crate::email_client::EmailClient;
use crate::gmaps_api_client::GoogleMapsAPIClient;
use crate::routes::{
    add_contact,
    admin_delete_contact,
    admin_delete_review,
    admin_edit_contact,
    admin_edit_review,
    admin_get_recent_reviews,
    admin_get_reviews_by_user,
    approve_contact,
    change_password,
    confirm,
    generate_reset_token,
    get_contact_by_id,
    get_genres,
    get_pending_contacts,
    health_check, 
    log_in,
    log_out,
    private_contacts,
    public_contacts,
    reset_password,
    review_contact,
    reviews_for_contact,
    sign_up,
    user_delete_contact,
    user_delete_review,
    user_edit_contact,
    user_edit_review,
    user_get_reviews
};

pub struct Application {
    port:   u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let email_client    = configuration.email_client.client();

        let address  = format!("{}:{}", configuration.application.host, configuration.application.port);
        let listener = TcpListener::bind(address)?;
        let port     = listener.local_addr().unwrap().port();
        let server   = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
            configuration.jwt_settings,
            configuration.frontend_url,
            configuration.gmaps_client
        ).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await   
    }
}

pub struct ApplicationBaseUrl(pub String);

pub fn get_connection_pool(db_config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(db_config.with_db())
}

async fn run(
    listener:     TcpListener,
    db_pool:      PgPool,
    email_client: EmailClient,
    base_url:     String,
    hmac_secret:  Secret<String>,
    redis_uri:    Secret<String>,
    jwt_settings: JWTSettings,
    frontend_url: String,
    gmaps_client: GoogleMapsAPIClient,
) -> Result<Server, anyhow::Error> {
    let base_url     = web::Data::new(ApplicationBaseUrl(base_url));
    let db_pool      = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let jwt_settings = web::Data::new(jwt_settings);
    let gmaps_client = web::Data::new(gmaps_client);
    let _secret_key  = Key::from(hmac_secret.expose_secret().as_bytes()); // previously used for RedisSession. idk if necessary
    let redis_client = web::Data::new(
        redis::Client::open(redis_uri.expose_secret().clone())?
    );
    // let test_user = TestUser::generate(); // Use to create user first time running app
    // test_user.store(&db_pool).await;
    let server       = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
            ])
            .supports_credentials();

        // let cors = Cors::permissive(); // For debugging

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .route("/health_check", web::get().to(health_check))
            .route("/confirm", web::get().to(confirm))
            .route("/contact", web::get().to(get_contact_by_id))
            .route("/contacts", web::get().to(public_contacts))
            .route("/genres", web::get().to(get_genres))
            .route("/generate-reset-token", web::post().to(generate_reset_token))
            .route("/login", web::post().to(log_in))
            .route("/reset-password", web::post().to(reset_password))
            .route("/signup", web::post().to(sign_up))
            .route("/reviews", web::get().to(reviews_for_contact))
            .service(
                web::scope("/user")
                    .route("/add-contact", web::post().to(add_contact))
                    .route("/change-password", web::post().to(change_password))
                    .route("/logout", web::post().to(log_out))
                    .route("/private-contacts", web::get().to(private_contacts))
                    .route("/review-contact", web::post().to(review_contact))
                    .route("/delete-contact", web::post().to(user_delete_contact))
                    .route("/delete-review", web::post().to(user_delete_review))
                    .route("/edit-contact", web::post().to(user_edit_contact))
                    .route("/edit-review", web::post().to(user_edit_review))
                    .route("/my-reviews", web::get().to(user_get_reviews))
            )
            .service(
                web::scope("/admin")
                    .route("/pending-contacts", web::get().to(get_pending_contacts))
                    .route("/delete-contact", web::post().to(admin_delete_contact))
                    .route("/approve-pending-contact", web::post().to(approve_contact))
                    .route("/recent-reviews", web::get().to(admin_get_recent_reviews))
                    .route("/user-reviews", web::get().to(admin_get_reviews_by_user))
                    .route("/delete-review", web::post().to(admin_delete_review))
                    .route("/edit-contact", web::post().to(admin_edit_contact))
                    .route("/edit-review", web::post().to(admin_edit_review))
            )
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(jwt_settings.clone())
            .app_data(gmaps_client.clone())
            .app_data(base_url.clone())
            .app_data(redis_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

// TEMP: Use to create user first time running app

// pub struct TestUser {
//     pub user_id:  Uuid,
//     pub email:    String,
//     pub password: String
// }

// impl TestUser {
//     pub fn generate() -> Self {
//         Self {
//             user_id:  Uuid::new_v4(),
//             email:    "ccrsh@riseup.net".to_string(),
//             password: "password".to_string()
//         }
//     }

//     pub async fn store(&self, pool: &PgPool) {
//         let salt = SaltString::generate(&mut rand::thread_rng());
//         let hash = Argon2::new(
//             Algorithm::Argon2id,
//             Version::V0x13,
//             Params::new(15000, 2, 1, None).unwrap()
//         ).hash_password(self.password.as_bytes(), &salt)
//             .unwrap()
//             .to_string();
        
//         sqlx::query!(
//             "INSERT INTO users (user_id, email, password_hash) VALUES ($1, $2, $3)",
//             self.user_id,
//             self.email,
//             hash
//         ).execute(pool)
//             .await
//             .expect("Failed to store test user");
//     }
// }