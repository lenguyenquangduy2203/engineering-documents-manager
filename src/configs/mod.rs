pub mod dbc;
pub mod server;

use std::{env, sync::OnceLock};

use crate::configs::{dbc::DBCConfig, server::ServerConfig};

#[derive(Clone)]
pub struct Config {
    host: String,
    port: String,
    db_url: String,
}

static ENV: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn from_env() -> &'static Self {
        ENV.get_or_init(|| {
            dotenvy::dotenv().ok();

            let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
            let db_url =
                env::var("DB_URL").unwrap_or_else(|_| "sqlite://data/database.db".to_string());

            Self { host, port, db_url }
        })
    }
}

impl ServerConfig for Config {
    fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl DBCConfig for Config {
    fn get_db_url(&self) -> String {
        self.db_url.clone()
    }

    fn should_create_on_missing(&self) -> bool {
        true
    }
}
