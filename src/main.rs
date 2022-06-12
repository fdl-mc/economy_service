pub mod config;
pub mod service;
pub use config::Config;

pub mod proto;

use proto::economy::economy_server::EconomyServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = envy::from_env::<Config>().unwrap();

    let pool = sqlx::PgPool::connect(&config.database_url.to_owned())
        .await
        .unwrap();

    sqlx::migrate!().run(&pool.clone()).await.unwrap();

    let addr = "0.0.0.0:8000".parse().unwrap();
    let economy_service = service::EconomyService {};

    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("economy_service"))
        .add_service(EconomyServer::new(economy_service))
        .serve(addr)
        .await?;

    Ok(())
}
