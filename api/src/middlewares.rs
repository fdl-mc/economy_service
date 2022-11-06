use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use users_service_client::GetSelfResponse;

use crate::{responses::AppError, AppState};

pub(crate) async fn auth_middleware<B>(
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
