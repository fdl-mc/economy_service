use serde::Serialize;

#[derive(Debug, Serialize)]
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
