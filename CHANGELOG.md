# Changelog

## [Unreleased]

### Fixed

- turbo-stream content is no longer re-indented with the `indent` filter, which changed whitespace inside `<pre>` and `<textarea>` elements
- missing space between the `action` and `target` attributes of `<turbo-stream>` (introduced in 0.1.3)
- stray space in `<turbo-frame>` when no target is set
- the `<turbo-frame>` `target` attribute is now HTML-escaped

### Changed

- upgrade to Rust edition 2024; minimum supported Rust version is 1.88 (required by askama 0.16)
- the crate's templates no longer use the `optional_attribute` filter; it remains public for existing users

### Added

- README, `LICENSE-MIT` and `LICENSE-APACHE` files (dual MIT/Apache-2.0), and package metadata
- GitHub Actions CI: fmt, clippy, tests, MSRV check
- integration tests

## [0.2.0] - 2026-07-29

### Changed

- **Breaking:** upgrade askama from 0.12 to 0.16, replacing `askama_axum` with `askama_web`
- **Breaking:** upgrade axum from 0.7 to 0.8
- `optional_attribute` filter updated for the askama 0.16 filter API
- the extractors no longer depend on `async-trait`

## [0.1.4] - 2026-05-19

### Added

- `event` action (`TurboStream::event`, `TurboStreamBuilder::event`)

### Fixed

- `TurboStream::refresh` no longer takes an unused generic type parameter

## [0.1.3] - 2026-04-01

### Added

- `prepend`, `before`, `after` and `refresh` actions on `TurboStream` and `TurboStreamBuilder`

### Changed

- **Breaking:** `TurboStreamElement.target` is now `Option<String>`, and the `target` attribute is omitted when it is `None`
- the `<template>` element is omitted when there is no item

## [0.1.2] - 2023-12-14

### Changed

- **Breaking:** upgrade axum from 0.6 to 0.7 (and `askama_axum` to 0.4)

### Fixed

- turbo-stream template applies `indent` before `safe` (was `safe|indent`)

## [0.1.1] - 2023-12-01

### Changed

- turbo-stream content is indented inside `<template>`, and surrounding blank lines are trimmed

## [0.1.0] - 2023-11-17

Initial release.
