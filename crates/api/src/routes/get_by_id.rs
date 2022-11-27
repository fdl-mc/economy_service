use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use economy_service_core::get_or_create_economy_state;

use crate::{responses::AppError, AppState};

/// Fetch economy state of user by their ID
#[utoipa::path(
    get, path = "/{id}", tag = "Economy state",
    params(
        ("id" = String, Path)
    ),
    responses(
        (status = 200, body = EconomyState),
        (status = 404, body = AppError),
    ),
)]

pub(crate) async fn get_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
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
