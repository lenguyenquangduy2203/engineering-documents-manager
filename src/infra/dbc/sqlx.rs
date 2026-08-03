use std::{str::FromStr, fmt::Debug};

use anyhow::Ok;
use sqlx::{Pool, QueryBuilder, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};

use crate::configs::{self, dbc::DBCConfig};

pub async fn get_conn() -> anyhow::Result<Pool<Sqlite>> {
    let config = configs::Config::from_env();

    let pool = SqlitePool::connect_with(SqliteConnectOptions::from_str(&config.get_db_url())?
        .create_if_missing(config.should_create_on_missing())
    ).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub trait FilterSpecification: Debug + Send + Sync {
    fn apply(&self, builder: &mut QueryBuilder<Sqlite>);
}