use crate::{extractors::AuthenticatedUser, responses::AppError, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use economy_service_core::get_or_create_economy_state;

/// Fetch your economy state data
#[utoipa::path(
    get, path = "/me", tag = "Economy state",
    responses(
        (status = 200, body = EconomyState, description = "Successful fetch"),
        (status = 401, body = AppError, description = "Authentication failed"),
    ),
    security(("api_key" = []))
)]
pub(crate) async fn get_self(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
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
