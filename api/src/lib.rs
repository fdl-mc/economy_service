use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use economy_service_core::get_or_create_economy_state;
use economy_service_migration::{
    sea_orm::{Database, DbConn},
    Migrator, MigratorTrait,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use users_service_client::{GetSelfResponse, User, UsersServiceClient};

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,
    users_service_url: String,
}

#[derive(Debug)]
struct AppState {
    users_client: UsersServiceClient,
    conn: DbConn,
}

#[derive(Debug, Serialize)]
struct AppError {
    detail: String,
}
impl AppError {
    fn new(detail: impl Into<String>) -> Self {
        AppError {
            detail: detail.into(),
        }
    }
}

async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Extract token from header
    let token = req
        .headers()
        .get("x-token")
        .and_then(|t| t.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(AppError::new("No token provided")),
        ))?;

    // Get users service client from extensions
    let client = &req
        .extensions()
        .get::<Arc<AppState>>()
        .unwrap()
        .users_client;

    // Get user
    let res = client.get_self(token).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppError::new(err.to_string())),
        )
    })?;

    let user = match res {
        GetSelfResponse::Ok(user) => user,
        GetSelfResponse::Unauthenticated => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AppError::new("Authentication failed")),
            ))
        }
    };

    // Insert user into extensions
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

async fn get_self(user: Extension<User>, state: Extension<Arc<AppState>>) -> impl IntoResponse {
    get_or_create_economy_state(user.id, &state.conn)
        .await
        .map(Json)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppError::new(err.to_string())),
            )
        })
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
        .route(
            "/me",
            get(get_self).layer(middleware::from_fn(auth_middleware)),
        )
        .layer(Extension(Arc::new(state)))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
