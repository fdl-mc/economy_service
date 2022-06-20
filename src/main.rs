pub mod config;
pub mod models;
pub mod service;
pub use config::Config;

pub mod proto;

use proto::economy::economy_server::EconomyServer;
use proto::users::users_client::UsersClient;

use sea_orm::Database;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = envy::from_env::<Config>().unwrap();

    let conn = Database::connect(&config.database_url).await?;

    let users_client = UsersClient::connect(config.users_service_url.to_owned()).await?;

    let addr = "0.0.0.0:8000".parse().unwrap();
    let economy_service = service::EconomyService {
        config,
        conn,
        users_client,
    };

    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("economy_service"))
        .add_service(EconomyServer::new(economy_service))
        .serve(addr)
        .await?;

    Ok(())
}
