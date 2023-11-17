use std::fmt::Display;

use askama::Template;
use axum::response::IntoResponse;

use crate::{
    turbo_page::TurboPage,
    turbo_stream::{TurboStreamAction, TurboStreamElement},
};

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: target.into(),
            action: TurboStreamAction::Append,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    pub fn update<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: target.into(),
            action: TurboStreamAction::Update,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    pub fn replace<T>(mut self, target: &str, item: &T) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: target.into(),
            action: TurboStreamAction::Replace,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    pub fn replace_optional<T>(mut self, target: &str, item: &Option<T>) -> Result<Self, askama::Error>
    where
        T: Display,
    {
        if let Some(item) = item {
            let element = TurboStreamElement {
                item: Some(item),
                target: target.into(),
                action: TurboStreamAction::Replace,
            };
            self.elements.push(element.render()?);
        }
        Ok(self)
    }

    pub fn remove(mut self, target: &str) -> Result<Self, askama::Error> {
        let element = TurboStreamElement::<String> {
            item: None,
            target: target.into(),
            action: TurboStreamAction::Remove,
        };
        self.elements.push(element.render()?);
        Ok(self)
    }

    pub fn build(&self) -> impl IntoResponse {
        let html = self.elements.join("\n");
        TurboPage::new(html).into_response()
    }
}
