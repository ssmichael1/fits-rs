use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::KeywordValue;

#[derive(Debug, Clone)]
pub struct Table {}

impl Table {
    pub fn from_bytes(
        header: &Header,
        _rawbytes: &[u8],
    ) -> Result<(HDUData, usize), Box<dyn std::error::Error>> {
        // Section 7.2 of the fits standard 4.0 manual
        // Note: this is an objectively awful way to store a table
        // but it is the standard

        // Check bitpix is 8
        let kwbitpix = header
            .get(1)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwbitpix.name != "BITPIX" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwbitpix.name.clone(),
                1,
            )));
        }
        match &kwbitpix.value {
            KeywordValue::Int(value) => {
                if *value != 8 {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid BITPIX value".to_string(),
                    )));
                }
            }
            _ => {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid BITPIX value".to_string(),
                ))
                .into());
            }
        }

        // Check naxis is 2
        let kwaxes = header
            .get(2)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwaxes.name != "NAXIS" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwaxes.name.clone(),
                2,
            )));
        }
        match &kwaxes.value {
            KeywordValue::Int(value) => {
                if *value != 2 {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid NAXIS value".to_string(),
                    )));
                }
            }
            _ => {
                return Err(
                    Box::new(HeaderError::GenericError("Invalid NAXIS value".to_string())).into(),
                );
            }
        }

        // get naxis1 and naxis2
        let kwaxis1 = header
            .get(3)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwaxis1.name != "NAXIS1" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwaxis1.name.clone(),
                3,
            )));
        }
        let nrowchars = match &kwaxis1.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid NROWCHARS (NAXIS1) value".to_string(),
                )));
            }
        };
        let kwaxis2 = header
            .get(4)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwaxis2.name != "NAXIS2" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwaxis2.name.clone(),
                4,
            )));
        }
        let nrows = match &kwaxis2.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid NROWS (NAXIS2) value".to_string(),
                )));
            }
        };

        Ok((HDUData::Table(Box::new(Table {})), nrows * nrowchars))
    }
}
