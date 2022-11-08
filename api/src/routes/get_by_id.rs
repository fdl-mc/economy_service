use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use economy_service_core::get_or_create_economy_state;
use std::sync::Arc;

use crate::{responses::AppError, AppState};

pub(crate) fn get_by_id() -> Router {
    async fn handler(
        Path(id): Path<i32>,
        state: Extension<Arc<AppState>>,
    ) -> Result<impl IntoResponse, impl IntoResponse> {
        let res = state.users_client.get_user(id).await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppError::new(err.to_string())),
            )
        })?;

        let user = match res {
            users_service_client::GetUserResponse::Ok(user) => user,
            users_service_client::GetUserResponse::NotFound => {
                return Err((StatusCode::NOT_FOUND, Json(AppError::new("User not found"))))
            }
            _ => unreachable!(),
        };

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

    Router::new().route("/:id", get(handler))
}
