//! The [`AcceptTurboStream`] extractor.

use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::request::Parts};

/// Extractor that is `true` when the request's `Accept` header includes `text/vnd.turbo-stream.html`.
///
/// Turbo sends that header for form submissions other than GET, so a handler can answer with a Turbo Stream
/// or fall back to a redirect or full page. Never rejects the request.
pub struct AcceptTurboStream(pub bool);

impl<S> FromRequestParts<S> for AcceptTurboStream
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let accept_header = parts.headers.get("accept");

        let Some(accept_header) = accept_header else {
            return Ok(AcceptTurboStream(false));
        };

        let Ok(accept_header) = accept_header.to_str() else {
            return Ok(AcceptTurboStream(false));
        };

        if !accept_header.contains("text/vnd.turbo-stream.html") {
            return Ok(AcceptTurboStream(false));
        };

        Ok(AcceptTurboStream(true))
    }
}
