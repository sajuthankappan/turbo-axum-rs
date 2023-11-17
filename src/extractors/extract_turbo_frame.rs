use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct ExtractTurboFrame(pub Option<String>);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractTurboFrame
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let turbo_frame_header = parts.headers.get("turbo-frame");

        let Some(turbo_frame_header) = turbo_frame_header else {
            return Ok(ExtractTurboFrame(None));
        };

        let Ok(turbo_frame_header) = turbo_frame_header.to_str()  else {
            return  Ok(ExtractTurboFrame(None));
        };

        Ok(ExtractTurboFrame(Some(turbo_frame_header.into())))
    }
}
