mod common;

use common::{read_turbo_stream, stream};
use turbo_axum::turbo_stream::TurboStream;

#[tokio::test]
async fn empty_builder_renders_empty_stream() {
    let body = read_turbo_stream(TurboStream::builder().build()).await;
    assert_eq!(body, "");
}

#[tokio::test]
async fn builder_joins_all_actions_in_order() -> Result<(), askama::Error> {
    let item = "<p>x</p>";
    let response = TurboStream::builder()
        .append("t1", &item)?
        .prepend("t2", &item)?
        .replace("t3", &item)?
        .update("t4", &item)?
        .remove("t5")?
        .before("t6", &item)?
        .after("t7", &item)?
        .event("t8", &item)?
        .refresh()?
        .build();

    let body = read_turbo_stream(response).await;
    let expected = [
        stream("append", Some("t1"), Some(item)),
        stream("prepend", Some("t2"), Some(item)),
        stream("replace", Some("t3"), Some(item)),
        stream("update", Some("t4"), Some(item)),
        stream("remove", Some("t5"), None),
        stream("before", Some("t6"), Some(item)),
        stream("after", Some("t7"), Some(item)),
        stream("event", Some("t8"), Some(item)),
        stream("refresh", None, None),
    ]
    .join("\n");
    assert_eq!(body, expected);
    Ok(())
}

#[tokio::test]
async fn replace_optional_skips_none() -> Result<(), askama::Error> {
    let response = TurboStream::builder()
        .replace_optional("a", &Some("<i>a</i>"))?
        .replace_optional::<&str>("b", &None)?
        .build();

    let body = read_turbo_stream(response).await;
    assert_eq!(body, stream("replace", Some("a"), Some("<i>a</i>")));
    Ok(())
}

#[tokio::test]
async fn build_can_be_called_more_than_once() -> Result<(), askama::Error> {
    let builder = TurboStream::builder().remove("a")?;
    let first = read_turbo_stream(builder.build()).await;
    let second = read_turbo_stream(builder.build()).await;
    assert_eq!(first, second);
    Ok(())
}
