use crate::KeywordValue;
use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum FITSError {
    #[error("Invalid header")]
    InvalidHeader,
    #[error("Bad Keyword Length: {0}")]
    BadKeywordLength(usize),
    #[error("Invalid Character in Keyword: \"{0}\"")]
    InvalidCharacterInKeyword(String),
    #[error("Invalid Keyword Record: {0}")]
    InvalidKeywordRecord(String),
    #[error("Unsupported Extension: {0}")]
    UnspportedExtension(String),
    #[error("Generic Error: {0}")]
    GenericError(String),
    #[error("Invalid Keyword Placement: {0} at {1}")]
    InvalidKeywordPlacement(String, usize),
    #[error("Unsupported Extension: {0}")]
    UnsupportedExtension(String),
    #[error("Unexpected Value Type in Keyword {0}")]
    UnexpectedValueType(String),
    #[error("Unexpected Keyword Value: {0} = {1}")]
    UnexpectedKeywordValue(String, KeywordValue),
    #[error("Missing Keyword: {0}")]
    MissingKeyword(String),
    #[error("Invalid TForm string: {0}")]
    InvalidTForm(String),
    #[error("Invalid Data Bytes Available: Expected {0}, got {1}")]
    InvalidDataSize(usize, usize),
    #[error("Invalid Row: {0} ; Max Row: {1}")]
    InvalidRow(usize, usize),
    #[error("Invalid Column: {0} ; Max Column: {1}")]
    InvalidColumn(usize, usize),
}
