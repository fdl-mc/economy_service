use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct AppError {
    detail: String,
}
impl AppError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        AppError {
            detail: detail.into(),
        }
    }
}
