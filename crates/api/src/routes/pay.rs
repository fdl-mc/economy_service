use crate::{extractors::AuthenticatedUser, responses::AppError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use economy_service_core::{
    get_or_create_economy_state, update_economy_state, UpdateEconomyStateForm,
};
use economy_service_entity::economy_state;
use serde::Deserialize;
use users_service_client::GetUserResponse;
use utoipa::ToSchema;

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
        (status = 400, body = AppError, description = "Validation failed: invalid amount or payee is self or insufficient funds"),
        (status = 401, body = AppError, description = "Authentication failed"),
        (status = 404, body = AppError, description = "User not found"),
    ),
    security(("api_key" = []))
)]
pub(crate) async fn pay(
    State(state): State<AppState>,
    AuthenticatedUser(payer_user): AuthenticatedUser,
    Path(payee_id): Path<i32>,
    Json(data): Json<DataPay>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // validate amount
    if data.amount <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AppError::new("Amount should be more than 0")),
        ));
    }

    // check whether payer is not payee
    if payer_user.id == payee_id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AppError::new("Cannot pay to yourself")),
        ));
    }

    // fetch payee (just to check whether they exist or not)
    match state.users_client.get_user(payee_id).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppError::new(err.to_string())),
        )
    })? {
        GetUserResponse::Ok(_) => (),
        GetUserResponse::NotFound => {
            return Err((StatusCode::NOT_FOUND, Json(AppError::new("User not found"))))
        }
        _ => unreachable!(),
    };

    // get payer state
    let payer_state = get_or_create_economy_state(payer_user.id, &state.conn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppError::new(err.to_string())),
            )
        })?;

    // check whether payer has enough money
    if payer_state.balance < data.amount {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AppError::new("Insufficient funds")),
        ));
    }

    // get payer state
    let payee_state = get_or_create_economy_state(payee_id, &state.conn)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppError::new(err.to_string())),
            )
        })?;

    // Update payer balance
    let payer_state: economy_state::ActiveModel = payer_state.into();
    update_economy_state(
        payer_state.to_owned(),
        UpdateEconomyStateForm {
            balance: Some(payer_state.balance.unwrap() - data.amount),
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

    // Update payee balance
    let payee_state: economy_state::ActiveModel = payee_state.into();
    update_economy_state(
        payee_state.to_owned(),
        UpdateEconomyStateForm {
            balance: Some(payee_state.balance.unwrap() + data.amount),
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

    Ok(StatusCode::NO_CONTENT)
}
