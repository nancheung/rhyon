use crate::infrastructure::persistence::db::config;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let config = config::load();

    let db = Database::connect(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;
    Ok(db)
}
