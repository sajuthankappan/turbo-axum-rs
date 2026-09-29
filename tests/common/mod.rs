#![allow(dead_code)]

use axum::{
    body::to_bytes,
    http::{Request, header, request::Parts},
    response::{IntoResponse, Response},
};

pub const TURBO_STREAM_CONTENT_TYPE: &str = "text/vnd.turbo-stream.html";

/// Converts a response and returns its `Content-Type` header and body.
pub async fn read_response(response: impl IntoResponse) -> (Option<String>, String) {
    let response: Response = response.into_response();
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().to_string());
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (content_type, String::from_utf8(body.to_vec()).unwrap())
}

/// Reads a response that must be a turbo stream and returns its body.
pub async fn read_turbo_stream(response: impl IntoResponse) -> String {
    let (content_type, body) = read_response(response).await;
    assert_eq!(content_type.as_deref(), Some(TURBO_STREAM_CONTENT_TYPE));
    body
}

/// Expected markup for one `<turbo-stream>` element.
pub fn stream(action: &str, target: Option<&str>, item: Option<&str>) -> String {
    let target = target
        .map(|t| format!(r#" target="{t}""#))
        .unwrap_or_default();
    match item {
        Some(item) => format!(
            "<turbo-stream action=\"{action}\"{target}>\n  <template>\n    {item}\n  </template>\n</turbo-stream>"
        ),
        None => format!("<turbo-stream action=\"{action}\"{target}>\n</turbo-stream>"),
    }
}

pub fn request_parts(headers: &[(&str, &str)]) -> Parts {
    let mut builder = Request::builder();
    for (name, value) in headers {
        builder = builder.header(*name, *value);
    }
    builder.body(()).unwrap().into_parts().0
}
