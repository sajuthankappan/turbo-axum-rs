use std::fmt::Display;

use askama::Template;

use crate::filters;

#[derive(Template)]
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

    pub fn with_target_top(frame_id: &str, content: T) -> Self
    where
        T: Display,
    {
        Self::with_target(frame_id, content, "_top")
    }

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

    pub fn element_id(&self) -> &str {
        &self.element_id
    }
}
