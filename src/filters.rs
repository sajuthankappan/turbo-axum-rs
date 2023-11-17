use std::fmt::Display;

pub fn optional_attribute<T>(s: &Option<T>, attribute_name: &str) -> ::askama::Result<String>
where
    T: Display,
{
    match s {
        Some(s) => Ok(format!(r#"{attribute_name}="{s}""#)),
        None => Ok(String::default()),
    }
}
