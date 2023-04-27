use secrecy::ExposeSecret;
use sea_orm::{ConnectOptions, Database};
use std::net::TcpListener;

use byot_server::configuration::get_configuration;
use byot_server::setup::{run};
use byot_server::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("byot_server".into(), "error".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config  = get_configuration().expect("Failed to read config");

    let db_url  = config.database.connection_string().expose_secret().to_owned();
    let db_info = ConnectOptions::new(db_url);
    let db_pool = Database::connect(db_info)
        .await
        .expect("Failed to connect to db");

    let address  = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, db_pool)?.await
}
