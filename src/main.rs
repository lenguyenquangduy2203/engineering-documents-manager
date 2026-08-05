use anyhow::Ok;

use engineering_documents_manager::web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    web::server::run().await?;

    Ok(())
}
