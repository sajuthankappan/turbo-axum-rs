//! The [`ExtractTurboFrame`] extractor.

use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::request::Parts};

/// Extractor for the `Turbo-Frame` request header: the ID of the frame that made the request,
/// or `None` when the request didn't come from a frame. Never rejects the request.
pub struct ExtractTurboFrame(pub Option<String>);

impl<S> FromRequestParts<S> for ExtractTurboFrame
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let turbo_frame_header = parts.headers.get("turbo-frame");

        let Some(turbo_frame_header) = turbo_frame_header else {
            return Ok(ExtractTurboFrame(None));
        };

        let Ok(turbo_frame_header) = turbo_frame_header.to_str() else {
            return Ok(ExtractTurboFrame(None));
        };

        Ok(ExtractTurboFrame(Some(turbo_frame_header.into())))
    }
}
