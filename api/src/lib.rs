pub(crate) mod middlewares;
mod openapi;
pub(crate) mod responses;
pub(crate) mod routes;

use axum::{Extension, Router};
use economy_service_migration::{
    sea_orm::{Database, DbConn},
    Migrator, MigratorTrait,
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use users_service_client::UsersServiceClient;

use crate::routes::{get_by_id, get_self, pay};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    database_url: String,
    users_service_url: String,
}

#[derive(Debug)]
pub(crate) struct AppState {
    users_client: UsersServiceClient,
    conn: DbConn,
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "economy_service_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = envy::from_env::<Config>().unwrap();

    let conn = Database::connect(&config.database_url).await.unwrap();
    let users_client = UsersServiceClient::new(&config.users_service_url);

    let state = AppState { users_client, conn };

    Migrator::up(&state.conn, None).await.unwrap();

    let app = Router::new().merge(openapi::ApiDoc::router()).merge(
        Router::new()
            .merge(get_self())
            .merge(get_by_id())
            .merge(pay())
            .layer(Extension(Arc::new(state)))
            .layer(TraceLayer::new_for_http()),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
