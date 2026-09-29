mod common;

use askama::Template;
use common::{read_turbo_stream, stream};
use turbo_axum::turbo_stream::{TurboStream, TurboStreamAction};

#[derive(Template)]
#[template(source = "<li>{{ title }}</li>", ext = "html")]
struct TodoItem {
    title: String,
}

#[tokio::test]
async fn append_renders_full_markup() {
    let body = read_turbo_stream(TurboStream::append("list", "<p>hi</p>")).await;
    assert_eq!(
        body,
        "<turbo-stream action=\"append\" target=\"list\">\n  <template>\n    <p>hi</p>\n  </template>\n</turbo-stream>"
    );
}

#[tokio::test]
async fn single_actions_with_item() {
    let cases = [
        (
            read_turbo_stream(TurboStream::append("t", "<p>x</p>")).await,
            "append",
        ),
        (
            read_turbo_stream(TurboStream::prepend("t", "<p>x</p>")).await,
            "prepend",
        ),
        (
            read_turbo_stream(TurboStream::replace("t", "<p>x</p>")).await,
            "replace",
        ),
        (
            read_turbo_stream(TurboStream::update("t", "<p>x</p>")).await,
            "update",
        ),
        (
            read_turbo_stream(TurboStream::before("t", "<p>x</p>")).await,
            "before",
        ),
        (
            read_turbo_stream(TurboStream::after("t", "<p>x</p>")).await,
            "after",
        ),
        (
            read_turbo_stream(TurboStream::event("t", "<p>x</p>")).await,
            "event",
        ),
    ];
    for (body, action) in cases {
        assert_eq!(
            body,
            stream(action, Some("t"), Some("<p>x</p>")),
            "action {action}"
        );
    }
}

#[tokio::test]
async fn remove_has_no_template() {
    let body = read_turbo_stream(TurboStream::remove("todo-1")).await;
    assert_eq!(body, stream("remove", Some("todo-1"), None));
}

#[tokio::test]
async fn refresh_has_no_target_or_template() {
    let body = read_turbo_stream(TurboStream::refresh()).await;
    assert_eq!(body, "<turbo-stream action=\"refresh\">\n</turbo-stream>");
}

#[tokio::test]
async fn askama_template_as_item() {
    let item = TodoItem {
        title: "Buy milk".into(),
    };
    let body = read_turbo_stream(TurboStream::append("todos", item)).await;
    assert_eq!(
        body,
        stream("append", Some("todos"), Some("<li>Buy milk</li>"))
    );
}

#[tokio::test]
async fn item_is_not_escaped() {
    let body = read_turbo_stream(TurboStream::update("t", "<b>&amp;</b>")).await;
    assert!(body.contains("<b>&amp;</b>"), "{body}");
}

#[tokio::test]
async fn target_is_escaped() {
    let body = read_turbo_stream(TurboStream::remove(r#"a"b"#)).await;
    assert!(!body.contains(r#"target="a"b""#), "{body}");
}

#[tokio::test]
async fn multiline_item_is_not_reindented() {
    let item = "<div>\n<pre>line1\nline2</pre>\n<textarea>a\nb</textarea>\n</div>";
    let body = read_turbo_stream(TurboStream::replace("t", item)).await;
    assert!(body.contains("<pre>line1\nline2</pre>"), "{body}");
    assert!(body.contains("<textarea>a\nb</textarea>"), "{body}");
}

#[tokio::test]
#[allow(deprecated)]
async fn replace_2_and_replace_3() {
    let body = read_turbo_stream(TurboStream::replace_2("a", "<i>1</i>", "b", "<i>2</i>")).await;
    assert_eq!(
        body,
        [
            stream("replace", Some("a"), Some("<i>1</i>")),
            stream("replace", Some("b"), Some("<i>2</i>")),
        ]
        .join("\n")
    );

    let body = read_turbo_stream(TurboStream::replace_3(
        "a", "<i>1</i>", "b", "<i>2</i>", "c", "<i>3</i>",
    ))
    .await;
    assert_eq!(
        body,
        [
            stream("replace", Some("a"), Some("<i>1</i>")),
            stream("replace", Some("b"), Some("<i>2</i>")),
            stream("replace", Some("c"), Some("<i>3</i>")),
        ]
        .join("\n")
    );
}

#[tokio::test]
#[allow(deprecated)]
async fn combined_actions() {
    let body = read_turbo_stream(TurboStream::remove_and_append("old", "<i>new</i>", "list")).await;
    assert_eq!(
        body,
        [
            stream("remove", Some("old"), None),
            stream("append", Some("list"), Some("<i>new</i>")),
        ]
        .join("\n")
    );

    let body = read_turbo_stream(TurboStream::replace_and_append(
        "<i>r</i>", "a", "<i>n</i>", "list",
    ))
    .await;
    assert_eq!(
        body,
        [
            stream("replace", Some("a"), Some("<i>r</i>")),
            stream("append", Some("list"), Some("<i>n</i>")),
        ]
        .join("\n")
    );

    let body = read_turbo_stream(TurboStream::replace_and_remove("<i>r</i>", "a", "old")).await;
    assert_eq!(
        body,
        [
            stream("replace", Some("a"), Some("<i>r</i>")),
            stream("remove", Some("old"), None),
        ]
        .join("\n")
    );

    let body = read_turbo_stream(TurboStream::replace_remove_and_append(
        "a", "<i>r</i>", "old", "list", "<i>n</i>",
    ))
    .await;
    assert_eq!(
        body,
        [
            stream("replace", Some("a"), Some("<i>r</i>")),
            stream("remove", Some("old"), None),
            stream("append", Some("list"), Some("<i>n</i>")),
        ]
        .join("\n")
    );

    let body = read_turbo_stream(TurboStream::remove_replace_and_append(
        "old", "<i>r</i>", "a", "<i>n</i>", "list",
    ))
    .await;
    assert_eq!(
        body,
        [
            stream("remove", Some("old"), None),
            stream("replace", Some("a"), Some("<i>r</i>")),
            stream("append", Some("list"), Some("<i>n</i>")),
        ]
        .join("\n")
    );
}

#[test]
fn action_display_matches_turbo_names() {
    let cases = [
        (TurboStreamAction::Append, "append"),
        (TurboStreamAction::Prepend, "prepend"),
        (TurboStreamAction::Update, "update"),
        (TurboStreamAction::Replace, "replace"),
        (TurboStreamAction::Remove, "remove"),
        (TurboStreamAction::Before, "before"),
        (TurboStreamAction::After, "after"),
        (TurboStreamAction::Refresh, "refresh"),
        (TurboStreamAction::Event, "event"),
    ];
    for (action, name) in cases {
        assert_eq!(action.to_string(), name);
    }
}
