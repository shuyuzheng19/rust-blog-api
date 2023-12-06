use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::conf::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbConfig {
    username: String,
    password: String,
    host: String,
    path: String,
    port: i32,
    max_content: u32,
}

impl DbConfig {
    pub async fn get_db(&self) -> Pool<Postgres> {
        let config = &CONFIG.db;
        let url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.path
        );
        let pool = PgPoolOptions::new()
            .max_connections(config.max_content)
            .connect(&url)
            .await
            .unwrap();
        return pool;
    }
}
