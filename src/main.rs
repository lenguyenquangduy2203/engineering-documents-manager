use anyhow::Ok;

mod configs;
mod web;
mod utils;
mod infra;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    web::server::run().await?;

    Ok(())
}
