use axum::response::{IntoResponse, Response};

pub mod error;

pub use error::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        format!("{}", self).into_response()
    }
}
