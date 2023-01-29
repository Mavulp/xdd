use std::ops::Deref;

use crate::error::Error;
use serde::{Deserialize, Deserializer};

pub(super) fn non_empty_trimmed_str<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<String>, D::Error> {
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()))
}

pub fn comma_string<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(Some(
            s.split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
        ));
    }

    Ok(None)
}

pub fn check_length(
    field_name: &'static str,
    field: Option<&str>,
    maximum_length: u64,
) -> Result<(), Error> {
    if let Some(field) = field {
        let field = &field.deref();
        if field.len() as u64 > maximum_length {
            return Err(Error::TooManyCharacters {
                field: field_name,
                maximum_length,
            });
        }
    }

    Ok(())
}
