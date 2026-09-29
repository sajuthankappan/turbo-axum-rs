# turbo-axum &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs] [![CI Status]][actions]
[Latest Version]: https://img.shields.io/crates/v/turbo-axum.svg
[crates.io]: https://crates.io/crates/turbo-axum
[Docs]: https://docs.rs/turbo-axum/badge.svg
[docs.rs]: https://docs.rs/turbo-axum
[CI Status]: https://github.com/sajuthankappan/turbo-axum-rs/actions/workflows/ci.yml/badge.svg?branch=main
[actions]: https://github.com/sajuthankappan/turbo-axum-rs/actions/workflows/ci.yml

**[Hotwire Turbo](https://turbo.hotwired.dev/) helpers for [axum](https://github.com/tokio-rs/axum)**

## Features

- Turbo Stream responses (`append`, `prepend`, `replace`, `update`, `remove`, `before`, `after`, `refresh`, `event`) served with the `text/vnd.turbo-stream.html` content type
- A builder to combine any number of stream actions in one response
- Turbo Frame rendering, with an optional `target`
- Extractors that detect Turbo Stream requests and read the `Turbo-Frame` request header
- Rendered with [askama](https://github.com/askama-rs/askama), so any askama template can be the content

## Installation

```toml
[dependencies]
turbo-axum = "0.3"
```

It targets axum 0.8 and askama 0.16, and requires Rust 1.88+.

## Usage example

Content is inserted as-is (it is not HTML-escaped), so pass already-rendered HTML, usually another askama template.

Respond with a single stream action

```rust
use turbo_axum::turbo_stream::TurboStream;

async fn create_todo() -> impl IntoResponse {
    let todo = TodoItem { title: "Buy milk".into() }; // any askama template
    TurboStream::append("todos", todo)
}
```

Combine several actions with the builder. Each step renders immediately and returns `Result<_, askama::Error>`.

```rust
use turbo_axum::turbo_stream::TurboStream;

// AppError: your error type, implementing IntoResponse and From<askama::Error>
async fn complete_todo() -> Result<impl IntoResponse, AppError> {
    let stream = TurboStream::builder()
        .remove("todo-1")?
        .update("todo-count", &"2 remaining")?
        .append("done", &done_item)?
        .build();
    Ok(stream)
}
```

Serve a Turbo Stream or a full page depending on the request

```rust
use turbo_axum::extractors::accept_turbo_stream::AcceptTurboStream;

async fn delete_todo(AcceptTurboStream(turbo): AcceptTurboStream) -> Response {
    if turbo {
        TurboStream::remove("todo-1").into_response()
    } else {
        Redirect::to("/todos").into_response()
    }
}
```

Render a Turbo Frame for frame requests

```rust
use turbo_axum::{extractors::extract_turbo_frame::ExtractTurboFrame, turbo_frame::TurboFrame};

async fn todos(ExtractTurboFrame(frame): ExtractTurboFrame) -> Response {
    let list = TodoList { todos: load_todos() };
    match frame {
        Some(frame_id) => TurboFrame::new(&frame_id, list).into_response(),
        None => TodosPage { list }.into_response(),
    }
}
```

`TurboFrame::with_target_top` renders a frame with `target="_top"`, so links inside it navigate the whole page.

## Maintainer

Maintained by [Saju Thankappan](https://github.com/sajuthankappan), creator of [Smito One](https://smito.in), which uses this crate for its Hotwire Turbo responses.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
