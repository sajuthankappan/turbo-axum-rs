use std::fmt::Display;

use askama::Values;

#[askama::filter_fn]
pub fn optional_attribute<T>(s: &Option<T>, _: &dyn Values, attribute_name: &str) -> ::askama::Result<String>
where
    T: Display,
{
    match s {
        Some(s) => Ok(format!(r#"{attribute_name}="{s}""#)),
        None => Ok(String::default()),
    }
}
