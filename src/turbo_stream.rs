//! Turbo Stream responses.

use std::fmt::Display;

use askama::Template;
use askama_web::WebTemplate;
use axum::response::IntoResponse;

use crate::{turbo_stream_builder::TurboStreamBuilder, turbo_stream_response::TurboStreamResponse};

/// Functions that build [Turbo Stream](https://turbo.hotwired.dev/handbook/streams) responses.
///
/// Each function returns a response with the `text/vnd.turbo-stream.html` content type.
/// Items are any [`Display`] value, usually an askama template, and are inserted without
/// HTML escaping, so pass already-rendered HTML. Targets are element IDs and are escaped.
///
/// Arguments are always `(target, item)`. To send more than one action, use [`TurboStream::builder`].
pub struct TurboStream {}

impl TurboStream {
    /// Starts a [`TurboStreamBuilder`] for a response with any number of actions.
    pub fn builder() -> TurboStreamBuilder {
        TurboStreamBuilder::new()
    }

    /// Appends `item` to the element with ID `target`.
    pub fn append<T>(target: &str, item: T) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Append,
        };
        Self::action(element)
    }

    /// Prepends `item` to the element with ID `target`.
    pub fn prepend<T>(target: &str, item: T) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Prepend,
        };
        Self::action(element)
    }

    /// Replaces the element with ID `target` with `item`.
    pub fn replace<T>(target: &str, item: T) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Replace,
        };
        Self::action(element)
    }

    /// Replaces two elements in one response.
    #[deprecated(
        since = "0.3.0",
        note = "use `TurboStream::builder().replace(..)?.replace(..)?.build()`"
    )]
    pub fn replace_2<T, U>(
        target1: &str,
        item1: T,
        target2: &str,
        item2: U,
    ) -> impl IntoResponse + use<T, U>
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item1),
            target: Some(target1.into()),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement {
            item: Some(item2),
            target: Some(target2.into()),
            action: TurboStreamAction::Replace,
        };
        Self::action_2(element1, element2)
    }

    /// Replaces three elements in one response.
    #[deprecated(
        since = "0.3.0",
        note = "use `TurboStream::builder().replace(..)?.replace(..)?.replace(..)?.build()`"
    )]
    pub fn replace_3<T, U, V>(
        target1: &str,
        item1: T,
        target2: &str,
        item2: U,
        target3: &str,
        item3: V,
    ) -> impl IntoResponse + use<T, U, V>
    where
        T: Display,
        U: Display,
        V: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item1),
            target: Some(target1.into()),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement {
            item: Some(item2),
            target: Some(target2.into()),
            action: TurboStreamAction::Replace,
        };
        let element3 = TurboStreamElement {
            item: Some(item3),
            target: Some(target3.into()),
            action: TurboStreamAction::Replace,
        };
        Self::action_3(element1, element2, element3)
    }

    /// Replaces the content of the element with ID `target` with `item`, keeping the element itself.
    pub fn update<T>(target: &str, item: T) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Update,
        };
        Self::action(element)
    }

    /// Removes the element with ID `target`.
    pub fn remove(target: &str) -> impl IntoResponse + use<> {
        let element = TurboStreamElement::<String> {
            item: None,
            target: Some(target.into()),
            action: TurboStreamAction::Remove,
        };
        Self::action(element)
    }

    /// Inserts `item` before the element with ID `target`.
    pub fn before<T>(target: &str, item: T) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Before,
        };
        Self::action(element)
    }

    /// Inserts `item` after the element with ID `target`.
    pub fn after<T>(target: &str, item: T) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::After,
        };
        Self::action(element)
    }

    /// Tells Turbo to refresh the current page.
    pub fn refresh() -> impl IntoResponse {
        let element = TurboStreamElement::<String> {
            item: None,
            target: None,
            action: TurboStreamAction::Refresh,
        };
        Self::action(element)
    }

    /// Sends an `event` action, a custom action that is not built into Turbo.
    ///
    /// The page must register a handler for it in `Turbo.StreamActions`.
    pub fn event<T>(target: &str, item: T) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: Some(target.into()),
            action: TurboStreamAction::Event,
        };
        Self::action(element)
    }

    /// Replaces, removes and appends in one response.
    #[deprecated(
        since = "0.3.0",
        note = "use `TurboStream::builder().replace(..)?.remove(..)?.append(..)?.build()`"
    )]
    pub fn replace_remove_and_append<T, U>(
        target_replace: &str,
        item_replace: T,
        target_remove: &str,
        target_append: &str,
        item_append: U,
    ) -> impl IntoResponse + use<T, U>
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item_replace),
            target: Some(target_replace.into()),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement::<String> {
            item: None,
            target: Some(target_remove.into()),
            action: TurboStreamAction::Remove,
        };
        let element3 = TurboStreamElement {
            item: Some(item_append),
            target: Some(target_append.into()),
            action: TurboStreamAction::Append,
        };
        Self::action_3(element1, element2, element3)
    }

    /// Removes and appends in one response.
    #[deprecated(
        since = "0.3.0",
        note = "use `TurboStream::builder().remove(..)?.append(..)?.build()`"
    )]
    pub fn remove_and_append<T>(
        target_remove: &str,
        item_append: T,
        target_append: &str,
    ) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element1 = TurboStreamElement::<String> {
            item: None,
            target: Some(target_remove.into()),
            action: TurboStreamAction::Remove,
        };
        let element2 = TurboStreamElement {
            item: Some(item_append),
            target: Some(target_append.into()),
            action: TurboStreamAction::Append,
        };
        Self::action_2(element1, element2)
    }

    /// Replaces and appends in one response.
    #[deprecated(
        since = "0.3.0",
        note = "use `TurboStream::builder().replace(..)?.append(..)?.build()`"
    )]
    pub fn replace_and_append<T, U>(
        item_replace: T,
        target_replace: &str,
        item_append: U,
        target_append: &str,
    ) -> impl IntoResponse + use<T, U>
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item_replace),
            target: Some(target_replace.into()),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement {
            item: Some(item_append),
            target: Some(target_append.into()),
            action: TurboStreamAction::Append,
        };
        Self::action_2(element1, element2)
    }

    /// Replaces and removes in one response.
    #[deprecated(
        since = "0.3.0",
        note = "use `TurboStream::builder().replace(..)?.remove(..)?.build()`"
    )]
    pub fn replace_and_remove<T>(
        item_replace: T,
        target_replace: &str,
        target_remove: &str,
    ) -> impl IntoResponse + use<T>
    where
        T: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item_replace),
            target: Some(target_replace.into()),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement::<String> {
            item: None,
            target: Some(target_remove.into()),
            action: TurboStreamAction::Remove,
        };
        Self::action_2(element1, element2)
    }

    /// Removes, replaces and appends in one response.
    #[deprecated(
        since = "0.3.0",
        note = "use `TurboStream::builder().remove(..)?.replace(..)?.append(..)?.build()`"
    )]
    pub fn remove_replace_and_append<T, U>(
        target_remove: &str,
        item_replace: T,
        target_replace: &str,
        item_append: U,
        target_append: &str,
    ) -> impl IntoResponse + use<T, U>
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement::<String> {
            item: None,
            target: Some(target_remove.into()),
            action: TurboStreamAction::Remove,
        };
        let element2 = TurboStreamElement {
            item: Some(item_replace),
            target: Some(target_replace.into()),
            action: TurboStreamAction::Replace,
        };
        let element3 = TurboStreamElement {
            item: Some(item_append),
            target: Some(target_append.into()),
            action: TurboStreamAction::Append,
        };
        Self::action_3(element1, element2, element3)
    }

    /// Responds with one [`TurboStreamElement`].
    pub fn action<T>(element: TurboStreamElement<T>) -> impl IntoResponse
    where
        T: Display,
    {
        TurboStreamResponse::new(element).into_response()
    }

    /// Responds with two [`TurboStreamElement`]s.
    pub fn action_2<T, U>(
        element1: TurboStreamElement<T>,
        element2: TurboStreamElement<U>,
    ) -> impl IntoResponse
    where
        T: Display,
        U: Display,
    {
        let element = TurboStreamTwoElements { element1, element2 };
        TurboStreamResponse::new(element).into_response()
    }

    /// Responds with three [`TurboStreamElement`]s.
    pub fn action_3<T, U, V>(
        element1: TurboStreamElement<T>,
        element2: TurboStreamElement<U>,
        element3: TurboStreamElement<V>,
    ) -> impl IntoResponse
    where
        T: Display,
        U: Display,
        V: Display,
    {
        let element = TurboStreamThreeElements {
            element1,
            element2,
            element3,
        };
        TurboStreamResponse::new(element).into_response()
    }
}

/// One `<turbo-stream>` element.
///
/// The item is wrapped in `<template>`. Both `target` and `item` are left out of the markup when `None`.
#[derive(Template, WebTemplate)]
#[template(path = "turbo-stream-element.html")]
pub struct TurboStreamElement<T>
where
    T: Display,
{
    /// ID of the element the action applies to.
    pub target: Option<String>,
    /// The Turbo Stream action.
    pub action: TurboStreamAction,
    /// Content for the action, inserted without HTML escaping.
    pub item: Option<T>,
}

/// Two `<turbo-stream>` elements rendered one after the other.
#[derive(Template, WebTemplate)]
#[template(path = "turbo-stream-two-elements.html")]
pub struct TurboStreamTwoElements<T, U>
where
    T: Display,
    U: Display,
{
    /// Element 1.
    pub element1: TurboStreamElement<T>,
    /// Element 2.
    pub element2: TurboStreamElement<U>,
}

/// Three `<turbo-stream>` elements rendered one after the other.
#[derive(Template, WebTemplate)]
#[template(path = "turbo-stream-three-elements.html")]
pub struct TurboStreamThreeElements<T, U, V>
where
    T: Display,
    U: Display,
    V: Display,
{
    /// Element 1.
    pub element1: TurboStreamElement<T>,
    /// Element 2.
    pub element2: TurboStreamElement<U>,
    /// Element 3.
    pub element3: TurboStreamElement<V>,
}

/// A Turbo Stream action. [`Display`] gives the name used in the `action` attribute, e.g. `append`.
#[derive(Debug)]
pub enum TurboStreamAction {
    /// Append to the target's content.
    Append,
    /// Prepend to the target's content.
    Prepend,
    /// Replace the target's content.
    Update,
    /// Replace the target element.
    Replace,
    /// Remove the target element.
    Remove,
    /// Insert before the target element.
    Before,
    /// Insert after the target element.
    After,
    /// Refresh the current page.
    Refresh,
    /// Custom `event` action, handled by the page.
    Event,
}

impl Display for TurboStreamAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = format!("{:?}", self).to_lowercase();
        write!(f, "{}", val)
    }
}
