//! askama filters.

// `#[askama::filter_fn]` generates helper items that can't carry docs
#![allow(missing_docs)]

use std::fmt::Display;

use askama::Values;

/// askama filter that renders `name="value"` for `Some(value)` and nothing for `None`.
///
/// The output is not escaped, so use it with `|safe` only for trusted values.
/// The crate's own templates no longer use it.
#[askama::filter_fn]
pub fn optional_attribute<T>(
    s: &Option<T>,
    _: &dyn Values,
    attribute_name: &str,
) -> ::askama::Result<String>
where
    T: Display,
{
    match s {
        Some(s) => Ok(format!(r#"{attribute_name}="{s}""#)),
        None => Ok(String::default()),
    }
}
