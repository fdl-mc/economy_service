use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::put, Extension, Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{responses::AppError, AppState};

#[derive(Deserialize, ToSchema)]
pub(crate) struct DataPay {
    /// Amount of money to pay
    amount: i32,

    /// Comment that will be shown to payee
    /// Currently unused
    #[allow(dead_code)]
    comment: Option<String>,
}

/// Pay money to other player
#[utoipa::path(
    put, path = "/{id}/pay", tag = "Economy state",
    request_body = DataPay,
    params(
        ("id" = String, Path, description = "Payee user ID")
    ),
    responses(
        (status = 204, description = "Successful payment"),
        (status = 400, body = AppError, description = "Insufficient funds"),
        (status = 401, body = AppError, description = "Authentication failed"),
        (status = 404, body = AppError, description = "User not found"),
    ),
)]
pub(crate) fn pay() -> Router {
    async fn handler(
        Json(data): Json<DataPay>,
        Path(id): Path<i32>,
        state: Extension<Arc<AppState>>,
    ) -> Result<(), impl IntoResponse> {
        Err((
            StatusCode::NOT_IMPLEMENTED,
            Json(AppError::new("Not implemented yet :)")),
        ))
    }

    Router::new().route("/:id/pay", put(handler))
}
