use serde::Serialize;
use utoipa::ToSchema;

/// Service error data
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AppError {
    /// Error detail message
    detail: String,
}
impl AppError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        AppError {
            detail: detail.into(),
        }
    }
}
