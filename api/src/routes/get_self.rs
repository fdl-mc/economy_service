use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use economy_service_core::get_or_create_economy_state;
use std::sync::Arc;
use users_service_client::User;

use crate::{responses::AppError, AppState};

pub(crate) fn get_self() -> Router {
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

    Router::new().route("/me", get(handler))
}
