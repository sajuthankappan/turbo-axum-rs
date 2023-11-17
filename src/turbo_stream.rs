use std::fmt::Display;

use askama::Template;
use axum::response::IntoResponse;

use crate::{turbo_page::TurboPage, turbo_stream_builder::TurboStreamBuilder};

pub struct TurboStream {}

impl TurboStream {
    pub fn builder() -> TurboStreamBuilder {
        TurboStreamBuilder::new()
    }

    pub fn append<T>(item: T, target: &str) -> impl IntoResponse
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: target.into(),
            action: TurboStreamAction::Append,
        };
        Self::action(element)
    }

    pub fn update<T>(target: &str, item: T) -> impl IntoResponse
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: target.into(),
            action: TurboStreamAction::Update,
        };
        Self::action(element)
    }

    pub fn replace<T>(target: &str, item: T) -> impl IntoResponse
    where
        T: Display,
    {
        let element = TurboStreamElement {
            item: Some(item),
            target: target.into(),
            action: TurboStreamAction::Replace,
        };
        Self::action(element)
    }

    pub fn replace_2<T, U>(target1: &str, item1: T, target2: &str, item2: U) -> impl IntoResponse
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item1),
            target: target1.into(),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement {
            item: Some(item2),
            target: target2.into(),
            action: TurboStreamAction::Replace,
        };
        Self::action_2(element1, element2)
    }

    pub fn replace_3<T, U, V>(target1: &str, item1: T, target2: &str, item2: U, target3: &str, item3: V) -> impl IntoResponse
    where
        T: Display,
        U: Display,
        V: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item1),
            target: target1.into(),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement {
            item: Some(item2),
            target: target2.into(),
            action: TurboStreamAction::Replace,
        };
        let element3 = TurboStreamElement {
            item: Some(item3),
            target: target3.into(),
            action: TurboStreamAction::Replace,
        };
        Self::action_3(element1, element2, element3)
    }

    pub fn remove(target: &str) -> impl IntoResponse {
        let element = TurboStreamElement::<String> {
            item: None,
            target: target.into(),
            action: TurboStreamAction::Remove,
        };
        Self::action(element)
    }

    pub fn replace_remove_and_append<T, U>(
        target_replace: &str,
        item_replace: T,
        target_remove: &str,
        target_append: &str,
        item_append: U,
    ) -> impl IntoResponse
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item_replace),
            target: target_replace.into(),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement::<String> {
            item: None,
            target: target_remove.into(),
            action: TurboStreamAction::Remove,
        };
        let element3 = TurboStreamElement {
            item: Some(item_append),
            target: target_append.into(),
            action: TurboStreamAction::Append,
        };
        Self::action_3(element1, element2, element3)
    }

    pub fn remove_and_append<T>(target_remove: &str, item_append: T, target_append: &str) -> impl IntoResponse
    where
        T: Display,
    {
        let element1 = TurboStreamElement::<String> {
            item: None,
            target: target_remove.into(),
            action: TurboStreamAction::Remove,
        };
        let element2 = TurboStreamElement {
            item: Some(item_append),
            target: target_append.into(),
            action: TurboStreamAction::Append,
        };
        Self::action_2(element1, element2)
    }

    pub fn replace_and_append<T, U>(item_replace: T, target_replace: &str, item_append: U, target_append: &str) -> impl IntoResponse
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item_replace),
            target: target_replace.into(),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement {
            item: Some(item_append),
            target: target_append.into(),
            action: TurboStreamAction::Append,
        };
        Self::action_2(element1, element2)
    }

    pub fn replace_and_remove<T>(item_replace: T, target_replace: &str, target_remove: &str) -> impl IntoResponse
    where
        T: Display,
    {
        let element1 = TurboStreamElement {
            item: Some(item_replace),
            target: target_replace.into(),
            action: TurboStreamAction::Replace,
        };
        let element2 = TurboStreamElement::<String> {
            item: None,
            target: target_remove.into(),
            action: TurboStreamAction::Remove,
        };
        Self::action_2(element1, element2)
    }

    pub fn remove_replace_and_append<T, U>(
        target_remove: &str,
        item_replace: T,
        target_replace: &str,
        item_append: U,
        target_append: &str,
    ) -> impl IntoResponse
    where
        T: Display,
        U: Display,
    {
        let element1 = TurboStreamElement::<String> {
            item: None,
            target: target_remove.into(),
            action: TurboStreamAction::Remove,
        };
        let element2 = TurboStreamElement {
            item: Some(item_replace),
            target: target_replace.into(),
            action: TurboStreamAction::Replace,
        };
        let element3 = TurboStreamElement {
            item: Some(item_append),
            target: target_append.into(),
            action: TurboStreamAction::Append,
        };
        Self::action_3(element1, element2, element3)
    }

    pub fn action<T>(element: TurboStreamElement<T>) -> impl IntoResponse
    where
        T: Display,
    {
        TurboPage::new(element).into_response()
    }

    pub fn action_2<T, U>(element1: TurboStreamElement<T>, element2: TurboStreamElement<U>) -> impl IntoResponse
    where
        T: Display,
        U: Display,
    {
        let element = TurboStreamTwoElements { element1, element2 };
        TurboPage::new(element).into_response()
    }

    pub fn action_3<T, U, V>(element1: TurboStreamElement<T>, element2: TurboStreamElement<U>, element3: TurboStreamElement<V>) -> impl IntoResponse
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
        TurboPage::new(element).into_response()
    }
}

#[derive(Template)]
#[template(path = "turbo-stream-element.html")]
pub struct TurboStreamElement<T>
where
    T: Display,
{
    pub target: String,
    pub action: TurboStreamAction,
    pub item: Option<T>,
}

#[derive(Template)]
#[template(path = "turbo-stream-two-elements.html")]
pub struct TurboStreamTwoElements<T, U>
where
    T: Display,
    U: Display,
{
    pub element1: TurboStreamElement<T>,
    pub element2: TurboStreamElement<U>,
}

#[derive(Template)]
#[template(path = "turbo-stream-three-elements.html")]
pub struct TurboStreamThreeElements<T, U, V>
where
    T: Display,
    U: Display,
    V: Display,
{
    pub element1: TurboStreamElement<T>,
    pub element2: TurboStreamElement<U>,
    pub element3: TurboStreamElement<V>,
}

#[derive(Debug)]
pub enum TurboStreamAction {
    Append,
    Update,
    Replace,
    Remove,
}

impl Display for TurboStreamAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = format!("{:?}", self).to_lowercase();
        write!(f, "{}", val)
    }
}
