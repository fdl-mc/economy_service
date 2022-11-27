use crate::{responses::AppError, AppState};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use users_service_client::{GetSelfResponse, User};

pub(crate) struct AuthenticatedUser(pub User);

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = (StatusCode, Json<AppError>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract token from header
        let token = parts
            .headers
            .get("x-token")
            .and_then(|t| t.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                Json(AppError::new("No token provided")),
            ))?;

        // Get user
        let res = state.users_client.get_self(token).await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppError::new(err.to_string())),
            )
        })?;

        match res {
            GetSelfResponse::Ok(user) => Ok(AuthenticatedUser(user)),
            GetSelfResponse::Unauthenticated => Err((
                StatusCode::UNAUTHORIZED,
                Json(AppError::new("Authentication failed")),
            )),
        }
    }
}
