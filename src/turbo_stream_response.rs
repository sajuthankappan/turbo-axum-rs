//! The Turbo Stream response type.

use axum::{
    http::{HeaderMap, header},
    response::{IntoResponse, Response},
};

/// Wraps a response and sets its content type to `text/vnd.turbo-stream.html`.
pub struct TurboStreamResponse<T>
where
    T: IntoResponse,
{
    /// The wrapped response.
    pub response: T,
}

impl<T> TurboStreamResponse<T>
where
    T: IntoResponse,
{
    /// Wraps `response`.
    pub fn new(response: T) -> Self {
        Self { response }
    }
}

impl<T> IntoResponse for TurboStreamResponse<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            "text/vnd.turbo-stream.html".parse().unwrap(),
        );
        (headers, self.response).into_response()
    }
}
