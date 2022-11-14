use axum::response::{Html, IntoResponse, Response};

pub mod error;

pub use error::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        format!("{}", self).into_response()
    }
}

pub mod templates;
pub use templates::hello_html;

pub fn render<F>(f: F) -> Html<&'static str>
where
    F: FnOnce(&mut Vec<u8>) -> Result<(), std::io::Error>,
{
    let mut buf = Vec::new();
    f(&mut buf).expect("Error rendering template");
    let html: String = String::from_utf8_lossy(&buf).into();

    Html(Box::leak(html.into_boxed_str()))
}
