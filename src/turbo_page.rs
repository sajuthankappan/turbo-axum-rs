use axum::{
    http::{header, HeaderMap},
    response::{IntoResponse, Response},
};

pub struct TurboPage<T>
where
    T: IntoResponse,
{
    pub response: T,
}

impl<T> TurboPage<T>
where
    T: IntoResponse,
{
    pub fn new(response: T) -> Self {
        Self { response }
    }
}

impl<T> IntoResponse for TurboPage<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "text/vnd.turbo-stream.html".parse().unwrap());
        (headers, self.response).into_response()
    }
}
