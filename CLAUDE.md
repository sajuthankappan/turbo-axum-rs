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
cargo test                 # no tests exist yet
cargo test <name>          # run a single test by name filter
```

Edition 2024, `rust-version = "1.85"`.

## What this is

`turbo-axum` is a small library crate that helps build [Hotwire Turbo](https://turbo.hotwired.dev/) responses from axum 0.8 handlers. It renders HTML with askama 0.16 and uses `askama_web` (feature `axum-0.8`) to turn templates into responses.

## Architecture

- **`TurboPage<T>`** (`turbo_page.rs`) wraps any `IntoResponse` and sets `Content-Type: text/vnd.turbo-stream.html`. Every turbo-stream response passes through it.
- **`TurboStreamElement<T>`** (`turbo_stream.rs`) is the askama template for one `<turbo-stream action=… target=…><template>…</template></turbo-stream>` (`templates/turbo-stream-element.html`). `TurboStreamTwoElements` and `TurboStreamThreeElements` concatenate 2 or 3 elements.
- There are two APIs for producing streams:
  - **`TurboStream`** has static convenience fns (`append`, `replace`, `replace_2`, `remove_and_append`, …). They build typed elements and go through `action`/`action_2`/`action_3`, so they are limited to 1–3 elements. Each element can have a different item type.
  - **`TurboStreamBuilder`** (`TurboStream::builder()`) renders each element to a `String` right away. That supports any number of elements, but every step returns `Result<Self, askama::Error>`. `build()` joins the rendered elements and wraps them in `TurboPage`.
  - When adding a new action, add it to both APIs.
- **`TurboStreamAction`**: `Display` lowercases the `Debug` name, so a variant's name must match the Turbo action name exactly (e.g. `Before` → `before`).
- **`TurboFrame<T>`** (`turbo_frame.rs`) renders `<turbo-frame id=… target=…>` and uses the custom `optional_attribute` askama filter from `filters.rs`. askama finds the filter through the `use crate::filters;` import in the module that declares the template.
- **Extractors** (`src/extractors/`): `AcceptTurboStream(bool)` checks the `Accept` header for `text/vnd.turbo-stream.html`, and `ExtractTurboFrame(Option<String>)` reads the `Turbo-Frame` request header. Neither one ever rejects a request.
- Item/content values are inserted with `|safe` (not escaped). Callers pass HTML that is already rendered, usually another askama template.
- Public fns that return `impl IntoResponse` use explicit `+ use<T, …>` so the result doesn't capture the lifetimes of `&str` arguments. Keep that pattern in new fns, or edition 2024 will make the returned value borrow from the arguments.
