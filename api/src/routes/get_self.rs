use axum::{
    http::StatusCode, middleware, response::IntoResponse, routing::get, Extension, Json, Router,
};
use economy_service_core::get_or_create_economy_state;
use std::sync::Arc;
use users_service_client::User;

use crate::{middlewares::auth_middleware, responses::AppError, AppState};

/// Fetch your economy state data
#[utoipa::path(
    get, path = "/me", tag = "Economy state",
    responses(
        (status = 200, body = EconomyState),
        (status = 401, body = AppError),
    ),
    security(("api_key" = []))
)]
pub fn get_self() -> Router {
    async fn handler(user: Extension<User>, state: Extension<Arc<AppState>>) -> impl IntoResponse {
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

    Router::new().route(
        "/me",
        get(handler).layer(middleware::from_fn(auth_middleware)),
    )
}
