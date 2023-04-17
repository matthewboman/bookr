use dotenv::dotenv;
use sea_orm::*;
use std::env;

pub(super) async fn connect_to_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db     = Database::connect(&db_url).await?;

    Ok(db)
}