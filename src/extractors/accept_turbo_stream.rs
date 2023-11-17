use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AcceptTurboStream(pub bool);

#[async_trait]
impl<S> FromRequestParts<S> for AcceptTurboStream
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let accept_header = parts.headers.get("accept");

        let Some(accept_header) = accept_header else {
            return  Ok(AcceptTurboStream(false));
        };

        let Ok(accept_header) = accept_header.to_str() else {
            return  Ok(AcceptTurboStream(false));
        };

        if !accept_header.contains("text/vnd.turbo-stream.html") {
            return Ok(AcceptTurboStream(false));
        };

        Ok(AcceptTurboStream(true))
    }
}
