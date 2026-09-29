//! [Hotwire Turbo](https://turbo.hotwired.dev/) helpers for [axum](https://docs.rs/axum),
//! rendered with [askama](https://docs.rs/askama).
//!
//! - [`TurboStream`](turbo_stream::TurboStream) and
//!   [`TurboStreamBuilder`](turbo_stream_builder::TurboStreamBuilder) build Turbo Stream
//!   responses (`append`, `replace`, `remove`, …).
//! - [`TurboFrame`](turbo_frame::TurboFrame) renders a `<turbo-frame>`.
//! - [`AcceptTurboStream`](extractors::accept_turbo_stream::AcceptTurboStream) and
//!   [`ExtractTurboFrame`](extractors::extract_turbo_frame::ExtractTurboFrame) read Turbo's
//!   request headers.
//!
//! Content is inserted without HTML escaping, so pass already-rendered HTML, usually another
//! askama template. Arguments are always `(target, item)`.
//!
//! ```
//! use askama::Template;
//! use axum::response::{IntoResponse, Redirect, Response};
//! use turbo_axum::{
//!     extractors::accept_turbo_stream::AcceptTurboStream, turbo_stream::TurboStream,
//! };
//!
//! #[derive(Template)]
//! #[template(source = "<li id=\"todo-{{ id }}\">{{ title }}</li>", ext = "html")]
//! struct TodoItem {
//!     id: u32,
//!     title: String,
//! }
//!
//! async fn create_todo(AcceptTurboStream(turbo): AcceptTurboStream) -> Response {
//!     let todo = TodoItem { id: 1, title: "Buy milk".into() };
//!     if turbo {
//!         TurboStream::append("todos", todo).into_response()
//!     } else {
//!         Redirect::to("/todos").into_response()
//!     }
//! }
//! # let _: axum::Router = axum::Router::new().route("/todos", axum::routing::post(create_todo));
//! ```

#![warn(missing_docs)]

pub mod extractors;
pub mod filters;
pub mod turbo_frame;
pub mod turbo_stream;
pub mod turbo_stream_builder;
pub mod turbo_stream_response;

/// Renamed to [`turbo_stream_response`].
#[deprecated(since = "0.3.0", note = "renamed to `turbo_stream_response`")]
pub mod turbo_page {
    /// Renamed to [`TurboStreamResponse`](crate::turbo_stream_response::TurboStreamResponse).
    #[deprecated(
        since = "0.3.0",
        note = "renamed to `turbo_stream_response::TurboStreamResponse`"
    )]
    pub type TurboPage<T> = crate::turbo_stream_response::TurboStreamResponse<T>;
}
