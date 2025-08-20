use super::config::load_db_config;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let config = load_db_config();

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    );

    tracing::debug!(
        "正在连接数据库: {}://{}:{}/{}",
        "postgres",
        config.host,
        config.port,
        config.database
    );

    let db = Database::connect(connection_string).await?;

    tracing::info!("✅ 数据库连接成功");
    Ok(db)
}
