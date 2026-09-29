//! Turbo Frame rendering.

use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;

/// A [`<turbo-frame>`](https://turbo.hotwired.dev/handbook/frames) element around `content`.
///
/// `content` is inserted without HTML escaping. As a response it is served as `text/html`.
#[derive(Template, WebTemplate)]
#[template(path = "turbo-frame.html")]
pub struct TurboFrame<T>
where
    T: Display,
{
    element_id: String,
    target: Option<String>,
    content: T,
}

impl<T> TurboFrame<T>
where
    T: Display,
{
    /// A frame with ID `frame_id`.
    pub fn new(frame_id: &str, content: T) -> Self
    where
        T: Display,
    {
        Self {
            element_id: frame_id.into(),
            target: None,
            content,
        }
    }

    /// A frame with `target="_top"`, so links and forms inside it navigate the whole page.
    pub fn with_target_top(frame_id: &str, content: T) -> Self
    where
        T: Display,
    {
        Self::with_target(frame_id, content, "_top")
    }

    /// A frame whose links and forms navigate the frame with ID `target`.
    pub fn with_target(frame_id: &str, content: T, target: &str) -> Self
    where
        T: Display,
    {
        Self {
            element_id: frame_id.into(),
            target: Some(target.into()),
            content,
        }
    }

    /// The frame's ID.
    pub fn element_id(&self) -> &str {
        &self.element_id
    }
}
