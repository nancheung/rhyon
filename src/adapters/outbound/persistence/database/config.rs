use crate::load_config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub fn load_db_config() -> DbConfig {
    load_config!(db, DbConfig)
}
