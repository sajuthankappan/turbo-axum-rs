mod common;

use axum::extract::FromRequestParts;
use common::request_parts;
use turbo_axum::extractors::{
    accept_turbo_stream::AcceptTurboStream, extract_turbo_frame::ExtractTurboFrame,
};

async fn accept_turbo_stream(headers: &[(&str, &str)]) -> bool {
    let mut parts = request_parts(headers);
    let Ok(AcceptTurboStream(accepts)) =
        AcceptTurboStream::from_request_parts(&mut parts, &()).await;
    accepts
}

async fn extract_turbo_frame(headers: &[(&str, &str)]) -> Option<String> {
    let mut parts = request_parts(headers);
    let Ok(ExtractTurboFrame(frame)) = ExtractTurboFrame::from_request_parts(&mut parts, &()).await;
    frame
}

#[tokio::test]
async fn accepts_turbo_stream() {
    let accept = "text/vnd.turbo-stream.html, text/html, application/xhtml+xml";
    assert!(accept_turbo_stream(&[("accept", accept)]).await);
}

#[tokio::test]
async fn does_not_accept_turbo_stream() {
    assert!(!accept_turbo_stream(&[("accept", "text/html")]).await);
}

#[tokio::test]
async fn missing_accept_header() {
    assert!(!accept_turbo_stream(&[]).await);
}

#[tokio::test]
async fn extracts_turbo_frame_header() {
    let frame = extract_turbo_frame(&[("turbo-frame", "todo-list")]).await;
    assert_eq!(frame.as_deref(), Some("todo-list"));
}

#[tokio::test]
async fn missing_turbo_frame_header() {
    assert_eq!(extract_turbo_frame(&[]).await, None);
}
