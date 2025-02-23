use crate::Header;
use crate::HeaderError;
use crate::KeywordValue;

use std::error::Error;

pub fn get_keyword_int_at_index(
    header: &Header,
    index: usize,
    name: &str,
) -> Result<i64, Box<dyn Error>> {
    if let Some(kw) = header.get(index) {
        if kw.name != name {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                format!("{} not {}", kw.name.clone(), name),
                index,
            )));
        }
        if let KeywordValue::Int(v) = &kw.value {
            Ok(*v)
        } else {
            Err(Box::new(HeaderError::UnexpectedValueType(kw.name.clone())))
        }
    } else {
        Err(Box::new(HeaderError::MissingKeyword(name.to_string())))
    }
}

pub fn check_int_keyword_at_index(
    header: &Header,
    index: usize,
    name: &str,
    value: i64,
) -> Result<(), Box<dyn Error>> {
    if let Some(kw) = header.get(index) {
        if kw.name != name {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                format!("{} not {}", kw.name.clone(), name),
                index,
            )));
        }
        if let KeywordValue::Int(v) = &kw.value {
            if *v != value {
                return Err(Box::new(HeaderError::GenericError(format!(
                    "Invalid value for keyword {}",
                    name
                ))));
            }
        } else {
            return Err(Box::new(HeaderError::UnexpectedValueType(kw.name.clone())));
        }
        Ok(())
    } else {
        Err(Box::new(HeaderError::MissingKeyword(name.to_string())))
    }
}
