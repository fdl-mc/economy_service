pub(crate) mod extractors;
pub(crate) mod openapi;
pub(crate) mod responses;
pub(crate) mod routes;

use axum::{
    routing::{get, patch, put},
    Router,
};
use economy_service_migration::{
    sea_orm::{Database, DbConn},
    Migrator, MigratorTrait,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use users_service_client::UsersServiceClient;

use crate::routes::{add_money, get_by_id, get_self, pay};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    database_url: String,
    users_service_url: String,
}

#[derive(Clone, Debug)]
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

    let app = Router::new()
        .merge(
            Router::new()
                .route("/:id", get(get_by_id))
                .route("/:id", patch(add_money))
                .route("/me", get(get_self))
                .route("/:id/pay", put(pay))
                .with_state(state),
        )
        .merge(openapi::ApiDoc::router().with_state(()))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8020));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
