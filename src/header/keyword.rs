use crate::HeaderError;

#[derive(Clone, Debug, PartialEq)]
pub enum KeywordValue {
    None,
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
    ComplexInt(i64, i64),
    ComplexFloat(f64, f64),
    Undefined,
}

impl std::fmt::Display for KeywordValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KeywordValue::None => write!(f, "None"),
            KeywordValue::Bool(b) => write!(f, "Bool: {}", b),
            KeywordValue::String(s) => write!(f, "String: \"{}\"", s),
            KeywordValue::Int(i) => write!(f, "Int: {}", i),
            KeywordValue::Float(fl) => write!(f, "Float: {}", fl),
            KeywordValue::ComplexInt(r, i) => write!(f, "Complex Int: ({}, {})", r, i),
            KeywordValue::ComplexFloat(r, i) => write!(f, "Complex Float: ({}, {})", r, i),
            KeywordValue::Undefined => write!(f, "Undefined"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Keyword {
    pub name: String,
    pub value: KeywordValue,
    pub comment: Option<String>,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)?;
        if let Some(comment) = &self.comment {
            write!(f, " :: {}", comment)?;
        }
        Ok(())
    }
}

impl Default for Keyword {
    fn default() -> Self {
        Keyword {
            name: String::new(),
            value: KeywordValue::None,
            comment: None,
        }
    }
}

impl Keyword {
    pub fn new(kwstr: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if kwstr.len() != 80 {
            return Err(Box::new(HeaderError::BadKeywordLength(kwstr.len())));
        }
        let kwname = &kwstr[0..8];

        // Check that the keyword name is valid
        for c in kwname {
            let c = *c as char;
            if !c.is_ascii_uppercase() && c != ' ' && !c.is_ascii_digit() && c != '_' && c != '-' {
                println!("here? {}", c as u8);
                for i in kwname {
                    println!("{}", i);
                }
                return Err(Box::new(HeaderError::InvalidCharacterInKeyword(
                    String::from_utf8(kwname.to_vec())?,
                )));
            }
        }

        let mut kwname = String::from_utf8(kwname.to_vec())?;

        // Check that keyword does not contain any intermediate spaces
        // per Section 4.1.2.1
        kwname = kwname.trim_ascii().to_string();
        if kwname.contains(' ') {
            println!("here");
            return Err(Box::new(HeaderError::InvalidCharacterInKeyword(kwname)));
        }

        // Construct the keyword to be returned later
        let mut kw = Keyword {
            name: kwname,
            value: KeywordValue::None,
            comment: None,
        };

        // Does this keyword have a value?
        if kwstr[8] == 61 && kwstr[9] == 32 {
            let kvchars = String::from_utf8(kwstr[10..].to_vec())?;

            // See if there is a string enclosed in single quotes
            if kvchars.starts_with('\'') {
                // find end quote, skipping double single quotes
                let mut end = 1;
                while end < kvchars.len() {
                    if kvchars.chars().nth(end).unwrap() == '\'' {
                        if end + 1 < kvchars.len() && kvchars.chars().nth(end + 1).unwrap() == '\''
                        {
                            end += 2;
                        } else {
                            break;
                        }
                    } else {
                        end += 1;
                    }
                }
                kw.value = KeywordValue::String(kvchars[1..end].to_string().trim().to_string());
                let remainder = kvchars[end..].to_string();
                // look for comment anywhere in remainder
                if let Some(pos) = remainder.find('/') {
                    if pos < remainder.len() - 1 {
                        kw.comment = Some(remainder[(pos + 1)..].to_string().trim().to_string());
                    }
                }
            }
            // look for boolean in 30th byte of keyword
            else if kwstr[29] == b'T' || kwstr[29] == b'F' {
                if kwstr[29] == b'T' {
                    kw.value = KeywordValue::Bool(true);
                } else {
                    kw.value = KeywordValue::Bool(false);
                }
                let remainder = kvchars[20..].to_string();
                if let Some(pos) = remainder.find('/') {
                    if pos < remainder.len() - 1 {
                        kw.comment = Some(remainder[(pos + 1)..].to_string().trim().to_string());
                    }
                }
            }
            // Look for integer or float or complex types
            else {
                let realstr = kvchars[0..20].to_string().trim_start().to_string();
                let mut complexstr = kvchars[20..].to_string().trim_start().to_string();
                // find if complex string has a comment,
                // if so, remove it
                if let Some(pos) = complexstr.find('/') {
                    complexstr = complexstr[0..pos].to_string();
                }

                let is_int = realstr
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == '+' || c == '-');

                let is_float = realstr.chars().all(|c| {
                    c.is_ascii_digit() || c == '.' || c == '+' || c == '-' || c == 'E' || c == 'D'
                });

                let is_complex_float = complexstr.chars().all(|c| {
                    c.is_ascii_digit()
                        || c == '.'
                        || c == '+'
                        || c == '-'
                        || c == 'E'
                        || c == 'D'
                        || c == ' '
                        || c == ','
                        || c == '('
                        || c == ')'
                });

                let is_complex_int = complexstr.chars().all(|c| {
                    c.is_ascii_digit()
                        || c == '+'
                        || c == '-'
                        || c == ' '
                        || c == ','
                        || c == '('
                        || c == ')'
                });

                if is_int {
                    kw.value = KeywordValue::Int(realstr.parse::<i64>()?);
                    let remainder = kvchars[20..].to_string();
                    if let Some(pos) = remainder.find('/') {
                        if pos < remainder.len() - 1 {
                            kw.comment =
                                Some(remainder[(pos + 1)..].to_string().trim().to_string());
                        }
                    }
                } else if is_float {
                    kw.value = KeywordValue::Float(realstr.parse::<f64>()?);
                    let remainder = kvchars[20..].to_string();
                    if let Some(pos) = remainder.find('/') {
                        if pos < remainder.len() - 1 {
                            kw.comment =
                                Some(remainder[(pos + 1)..].to_string().trim().to_string());
                        }
                    }
                } else if is_complex_int {
                    // look for integers in format (real, imag)
                    // Find string that starts with '(' and ends with ')'
                    let start = complexstr.find('(');
                    let end = complexstr.find(')');
                    if start.is_none() || end.is_none() {
                        return Err(Box::new(HeaderError::InvalidKeywordRecord(
                            String::from_utf8(kwstr.to_vec())?,
                        )));
                    }
                    let start = start.unwrap();
                    let end = end.unwrap();
                    if end < start {
                        return Err(Box::new(HeaderError::InvalidKeywordRecord(
                            String::from_utf8(kwstr.to_vec())?,
                        )));
                    }
                    let parts = complexstr[(start + 1)..end].split(",");
                    let parts = parts.map(|x| x.trim()).collect::<Vec<_>>();
                    if parts.len() != 2 {
                        return Err(Box::new(HeaderError::InvalidKeywordRecord(
                            String::from_utf8(kwstr.to_vec())?,
                        )));
                    }
                    let real = parts[0].parse::<i64>()?;
                    let imag = parts[1].parse::<i64>()?;
                    kw.value = KeywordValue::ComplexInt(real, imag);
                    let remainder = kvchars[20..].to_string();
                    if let Some(pos) = remainder.find('/') {
                        if pos < remainder.len() - 1 {
                            kw.comment =
                                Some(remainder[(pos + 1)..].to_string().trim().to_string());
                        }
                    }
                } else if is_complex_float {
                    // look for floats in format (real, imag)
                    // Find string that starts with '(' and ends with ')'
                    let start = complexstr.find('(');
                    let end = complexstr.find(')');
                    if start.is_none() || end.is_none() {
                        return Err(Box::new(HeaderError::InvalidKeywordRecord(
                            String::from_utf8(kwstr.to_vec())?,
                        )));
                    }
                    let start = start.unwrap();
                    let end = end.unwrap();
                    if end < start {
                        return Err(Box::new(HeaderError::InvalidKeywordRecord(
                            String::from_utf8(kwstr.to_vec())?,
                        )));
                    }
                    let parts = complexstr[(start + 1)..end].split(",");
                    let parts = parts.map(|x| x.trim()).collect::<Vec<_>>();
                    if parts.len() != 2 {
                        return Err(Box::new(HeaderError::InvalidKeywordRecord(
                            String::from_utf8(kwstr.to_vec())?,
                        )));
                    }
                    let real = parts[0].parse::<f64>()?;
                    let imag = parts[1].parse::<f64>()?;
                    kw.value = KeywordValue::ComplexFloat(real, imag);
                    let remainder = kvchars[20..].to_string();
                    if let Some(pos) = remainder.find('/') {
                        if pos < remainder.len() - 1 {
                            kw.comment =
                                Some(remainder[(pos + 1)..].to_string().trim().to_string());
                        }
                    }
                } else {
                    return Err(Box::new(HeaderError::InvalidKeywordRecord(
                        String::from_utf8(kwstr.to_vec())?,
                    )));
                }
            }
        }

        Ok(kw)
    }
}
