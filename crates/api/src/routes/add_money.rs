use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use economy_service_core::{
    get_or_create_economy_state, update_economy_state, UpdateEconomyStateForm,
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{extractors::AuthenticatedUser, responses::AppError, AppState};

/// Data used in pay operation
#[derive(Deserialize, ToSchema)]
pub(crate) struct DataAddMoney {
    /// Amount of money to add
    amount: i32,
}

/// Add money to target user. Bankers only.
#[utoipa::path(
    patch, path = "/{id}", tag = "Economy state", request_body = DataAddMoney,
    params(
        ("id" = String, Path, description = "Target user ID")
    ),
    responses(
        (status = 204, description = "Successful fetch"),
        (status = 401, body = AppError, description = "Authentication failed"),
        (status = 403, body = AppError, description = "Missing banker role"),
        (status = 404, body = AppError, description = "User not found"),
    ),
    security(("api_key" = []))
)]
pub(crate) async fn add_money(
    Path(id): Path<i32>,
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Json(data): Json<DataAddMoney>,
) -> Result<(), impl IntoResponse> {
    let is_banker = get_or_create_economy_state(user.id, &state.conn)
        .await
        .map(|state| state.banker)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppError::new(err.to_string())),
            )
        })?;

    if !is_banker {
        return Err((
            StatusCode::FORBIDDEN,
            Json(AppError::new("Missing banker role")),
        ));
    }

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

    let user_state = get_or_create_economy_state(user.id, &state.conn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppError::new(err.to_string())),
            )
        })?;
    let balance = user_state.balance;

    update_economy_state(
        user_state.into(),
        UpdateEconomyStateForm {
            balance: Some(balance + data.amount),
            ..Default::default()
        },
        &state.conn,
    )
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppError::new(err.to_string())),
        )
    })?;

    Ok(())
}
