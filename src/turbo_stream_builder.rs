//! Builder for Turbo Stream responses with any number of actions.

use std::fmt::Display;

use askama::Template;
use axum::response::IntoResponse;

use crate::{
    turbo_stream::{TurboStreamAction, TurboStreamElement},
    turbo_stream_response::TurboStreamResponse,
};

/// Builds a Turbo Stream response with any number of actions.
///
/// Each method renders its element right away, so it returns `Result<Self, askama::Error>`.
/// Methods take `(target, &item)`; items are inserted without HTML escaping.
/// Call [`build`](Self::build) to get the response.
///
/// ```
/// use turbo_axum::turbo_stream::TurboStream;
///
/// # fn main() -> Result<(), askama::Error> {
/// let response = TurboStream::builder()
///     .remove("todo-1")?
///     .update("todo-count", &"2 remaining")?
///     .build();
/// # Ok(())
/// # }
/// ```
pub struct TurboStreamBuilder {
    elements: Vec<String>,
}

impl Default for TurboStreamBuilder {
    fn default() -> Self {
        let elements = Vec::new();
        Self { elements }
    }
}

impl TurboStreamBuilder {
    /// Creates an empty builder. Same as [`TurboStream::builder`](crate::turbo_stream::TurboStream::builder).
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends `item` to the element with ID `target`.
    pub fn append<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Append,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Prepends `item` to the element with ID `target`.
    pub fn prepend<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Prepend,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Replaces the element with ID `target` with `item`.
    pub fn replace<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Replace,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Replaces the element with ID `target` with `item` if it is `Some`; does nothing for `None`.
    pub fn replace_optional<T>(
        mut self,
        target: &str,
        item: &Option<T>,
    ) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        if let Some(item) = item {
            let element = TurboStreamElement {
                item: Some(item),
                target: Some(target.into()),
                action: TurboStreamAction::Replace,
            };
            self.elements.push(element.render()?);
        }
        Ok(self)
    }

    /// Replaces the content of the element with ID `target` with `item`, keeping the element itself.
    pub fn update<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Update,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Removes the element with ID `target`.
    pub fn remove(mut self, target: &str) -> Result<Self, askama::Error> {
        let element = TurboStreamElement::<String> {
            item: None,
            target: Some(target.into()),
            action: TurboStreamAction::Remove,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Inserts `item` before the element with ID `target`.
    pub fn before<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Before,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Tells Turbo to refresh the current page.
    pub fn refresh(mut self) -> Result<Self, askama::Error> {
        let element = TurboStreamElement::<String> {
            item: None,
            target: None,
            action: TurboStreamAction::Refresh,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Inserts `item` after the element with ID `target`.
    pub fn after<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::After,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Sends a custom `event` action; the page must register a handler for it in `Turbo.StreamActions`.
    pub fn event<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Event,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    /// Returns the response with all actions added so far. Can be called more than once.
    pub fn build(&self) -> impl IntoResponse + use<> {
        let html = self.elements.join("\n");
        TurboStreamResponse::new(html).into_response()
    }
}
