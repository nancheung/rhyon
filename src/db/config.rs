use crate::load_section;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub fn load() -> DbConfig {
    load_section!(db, DbConfig)
}
