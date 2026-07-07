mod config;
mod web;

#[tokio::main]
async fn main() {
    web::server::run().await;
}
