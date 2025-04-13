use crate::load_section;
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DbConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl DbConfig {
    fn load() -> Self {
        load_section!(db, DbConfig)
    }
}

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let config = DbConfig::load();

    let db = Database::connect(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;
    Ok(db)
}
