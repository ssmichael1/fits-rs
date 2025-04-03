use crate::Header;
use crate::KeywordValue;

use anyhow::{anyhow, Result};

pub fn get_keyword_int_at_index(header: &Header, index: usize, name: &str) -> Result<i64> {
    if let Some(kw) = header.get(index) {
        if kw.name != name {
            return Err(anyhow!(
                "Invalid keyword placement: expected \"{}\", got \"{}\"",
                name,
                kw.name.clone()
            ));
        }
        if let KeywordValue::Int(v) = &kw.value {
            Ok(*v)
        } else {
            Err(anyhow!(
                "Invalid value type for keyword {}: expected Int, got {:?}",
                name,
                kw.value
            ))
        }
    } else {
        Err(anyhow!("Missing keyword {} at index {}", name, index))
    }
}

pub fn check_int_keyword_at_index(
    header: &Header,
    index: usize,
    name: &str,
    value: i64,
) -> Result<()> {
    if let Some(kw) = header.get(index) {
        if kw.name != name {
            return Err(anyhow!(
                "Invalid keyword placement: expected \"{}\", got \"{}\"",
                name,
                kw.name.clone()
            ));
        }
        if let KeywordValue::Int(v) = &kw.value {
            if *v != value {
                return Err(anyhow!(
                    "Invaild value for keyword {}: expected {}, got {}",
                    name,
                    value,
                    v
                ));
            }
        } else {
            return Err(anyhow!(
                "Invalid value type for keyword {}: expected Int, got {:?}",
                name,
                kw.value
            ));
        }
        Ok(())
    } else {
        Err(anyhow!("Missing keyword {} at index {}", name, index))
    }
}
