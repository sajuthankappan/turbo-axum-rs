# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Working agreement

- **Never commit without being explicitly asked.** The maintainer reviews all changes before committing. Leave changes uncommitted in the working tree.
- Solo-maintained repo: work directly on `main`. No feature branches, no PRs.

## Commands

```sh
cargo build --all-targets
cargo clippy --all-targets
cargo fmt                  # rustfmt style edition 2024
cargo test                 # integration tests in tests/, one file per module
cargo test <name>          # run a single test by name filter
```

Edition 2024, `rust-version = "1.88"` (required by askama 0.16).

## What this is

`turbo-axum` is a small library crate that helps build [Hotwire Turbo](https://turbo.hotwired.dev/) responses from axum 0.8 handlers. It renders HTML with askama 0.16 and uses `askama_web` (feature `axum-0.8`) to turn templates into responses.

## Architecture

- **`TurboStreamResponse<T>`** (`turbo_stream_response.rs`, formerly `TurboPage`, with a deprecated alias in `lib.rs`) wraps any `IntoResponse` and sets `Content-Type: text/vnd.turbo-stream.html`. Every turbo-stream response passes through it.
- **`TurboStreamElement<T>`** (`turbo_stream.rs`) is the askama template for one `<turbo-stream action=… target=…><template>…</template></turbo-stream>` (`templates/turbo-stream-element.html`). `TurboStreamTwoElements` and `TurboStreamThreeElements` concatenate 2 or 3 elements.
- There are two APIs for producing streams:
  - **`TurboStream`** has static convenience fns (`append`, `replace`, `remove`, …). They build typed elements and go through `action`/`action_2`/`action_3`, so they are limited to 1–3 elements. Each element can have a different item type.
  - **`TurboStreamBuilder`** (`TurboStream::builder()`) renders each element to a `String` right away. That supports any number of elements, but every step returns `Result<Self, askama::Error>`. `build()` joins the rendered elements and wraps them in `TurboStreamResponse`.
  - When adding a new action, add it to both APIs.
  - Arguments are always `(target, item)`. Items are `T: Display`, and `&str` is `Display`, so the compiler won't catch swapped arguments.
  - The multi-action fns (`replace_2`, `replace_3`, `remove_and_append`, …) are deprecated in favour of the builder; don't add new ones.
- **`TurboStreamAction`**: `Display` lowercases the `Debug` name, so a variant's name must match the Turbo action name exactly (e.g. `Before` → `before`).
- **`TurboFrame<T>`** (`turbo_frame.rs`) renders `<turbo-frame id=… target=…>`. `filters.rs` has a public `optional_attribute` askama filter; the crate's own templates no longer use it (they use `{%- if let … %}` so attributes are escaped), but it stays for backward compatibility. A template that uses a custom filter needs `use crate::filters;` in the module that declares it.
- **Extractors** (`src/extractors/`): `AcceptTurboStream(bool)` checks the `Accept` header for `text/vnd.turbo-stream.html`, and `ExtractTurboFrame(Option<String>)` reads the `Turbo-Frame` request header. Neither one ever rejects a request (`Rejection = Infallible`).
- Item/content values are inserted with `|safe` (not escaped). Callers pass HTML that is already rendered, usually another askama template.
- Public fns that return `impl IntoResponse` use explicit `+ use<T, …>` so the result doesn't capture the lifetimes of `&str` arguments. Keep that pattern in new fns, or edition 2024 will make the returned value borrow from the arguments.
