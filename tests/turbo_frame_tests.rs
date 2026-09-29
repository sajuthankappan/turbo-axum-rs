mod common;

use askama::Template;
use common::read_response;
use turbo_axum::{turbo_frame::TurboFrame, turbo_page::TurboPage};

#[test]
fn frame_without_target() {
    let frame = TurboFrame::new("list", "<ul></ul>");
    assert_eq!(frame.element_id(), "list");
    assert_eq!(
        frame.render().unwrap(),
        "<turbo-frame id=\"list\">\n  <ul></ul>\n</turbo-frame>"
    );
}

#[test]
fn frame_with_target() {
    let frame = TurboFrame::with_target("list", "<ul></ul>", "details");
    assert_eq!(
        frame.render().unwrap(),
        "<turbo-frame id=\"list\" target=\"details\">\n  <ul></ul>\n</turbo-frame>"
    );
}

#[test]
fn frame_with_target_top() {
    let frame = TurboFrame::with_target_top("list", "<ul></ul>");
    assert!(
        frame
            .render()
            .unwrap()
            .starts_with("<turbo-frame id=\"list\" target=\"_top\">"),
    );
}

#[test]
fn frame_target_is_escaped() {
    let frame = TurboFrame::with_target("list", "<ul></ul>", r#"a"b"#);
    assert!(!frame.render().unwrap().contains(r#"target="a"b""#));
}

#[tokio::test]
async fn frame_response_is_html() {
    let (content_type, body) = read_response(TurboFrame::new("list", "<ul></ul>")).await;
    assert!(content_type.unwrap().starts_with("text/html"));
    assert!(body.contains("<ul></ul>"));
}

#[tokio::test]
async fn turbo_page_sets_turbo_stream_content_type() {
    let (content_type, body) = read_response(TurboPage::new("<p>x</p>")).await;
    assert_eq!(
        content_type.as_deref(),
        Some(common::TURBO_STREAM_CONTENT_TYPE)
    );
    assert_eq!(body, "<p>x</p>");
}
