use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::KeywordValue;

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub nfields: usize,
    pub tcol: Vec<usize>,
}

impl Table {
    pub fn from_bytes(header: &Header, _rawbytes: &[u8]) -> Result<(HDUData, usize), HeaderError> {
        // Section 7.2 of the fits standard 4.0 manual
        // Note: this is an objectively awful way to store a table
        // but it is the standard

        let mut table = Box::new(Table::default());

        // Check bitpix is 8
        let kwbitpix = header
            .get(1)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwbitpix.name != "BITPIX" {
            return Err(HeaderError::InvalidKeywordPlacement(
                kwbitpix.name.clone(),
                1,
            ));
        }
        match &kwbitpix.value {
            KeywordValue::Int(value) => {
                if *value != 8 {
                    return Err(HeaderError::GenericError(
                        "Invalid BITPIX value".to_string(),
                    ));
                }
            }
            _ => {
                return Err(HeaderError::GenericError(
                    "Invalid BITPIX value".to_string(),
                ))
            }
        }

        // Check naxis is 2
        let kwaxes = header
            .get(2)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwaxes.name != "NAXIS" {
            return Err(HeaderError::InvalidKeywordPlacement(kwaxes.name.clone(), 2));
        }
        match &kwaxes.value {
            KeywordValue::Int(value) => {
                if *value != 2 {
                    return Err(HeaderError::GenericError("Invalid NAXIS value".to_string()));
                }
            }
            _ => {
                return Err(HeaderError::GenericError("Invalid NAXIS value".to_string()));
            }
        }

        // get naxis1 and naxis2
        let kwaxis1 = header
            .get(3)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwaxis1.name != "NAXIS1" {
            return Err(HeaderError::InvalidKeywordPlacement(
                kwaxis1.name.clone(),
                3,
            ));
        }
        let nrowchars = match &kwaxis1.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(HeaderError::GenericError(
                    "Invalid NROWCHARS (NAXIS1) value".to_string(),
                ));
            }
        };
        let kwaxis2 = header
            .get(4)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwaxis2.name != "NAXIS2" {
            return Err(HeaderError::InvalidKeywordPlacement(
                kwaxis2.name.clone(),
                4,
            ));
        }
        let nrows = match &kwaxis2.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(HeaderError::GenericError(
                    "Invalid NROWS (NAXIS2) value".to_string(),
                ));
            }
        };

        // pcount is next ... should be 0
        let kwpcount = header
            .get(5)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwpcount.name != "PCOUNT" {
            return Err(HeaderError::InvalidKeywordPlacement(
                kwpcount.name.clone(),
                5,
            ));
        }
        if let KeywordValue::Int(value) = &kwpcount.value {
            if *value != 0 {
                return Err(HeaderError::UnexpectedKeywordValue(
                    "PCOUNT".to_string(),
                    kwpcount.value.clone(),
                ));
            }
        } else {
            return Err(HeaderError::UnexpectedKeywordValue(
                "PCOUNT".to_string(),
                kwpcount.value.clone(),
            ));
        }

        // gcount is next ... should be 1
        let kwgcount = header
            .get(6)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwgcount.name != "GCOUNT" {
            return Err(HeaderError::InvalidKeywordPlacement(
                kwgcount.name.clone(),
                6,
            ));
        }
        if let KeywordValue::Int(value) = &kwgcount.value {
            if *value != 1 {
                return Err(HeaderError::UnexpectedKeywordValue(
                    "GCOUNT".to_string(),
                    kwgcount.value.clone(),
                ));
            }
        } else {
            return Err(HeaderError::UnexpectedKeywordValue(
                "GCOUNT".to_string(),
                kwgcount.value.clone(),
            ));
        }
        // TFIELDS is next;
        let kwtfields = header
            .get(7)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwtfields.name != "TFIELDS" {
            return Err(HeaderError::InvalidKeywordPlacement(
                kwtfields.name.clone(),
                7,
            ));
        }
        table.nfields = match &kwtfields.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(HeaderError::GenericError(
                    "Invalid NFIELDS (TFIELDS) value".to_string(),
                ));
            }
        };

        table.tcol = Vec::with_capacity(table.nfields);
        for i in 0..table.nfields {
            let kw = header
                .find(&format!("TBCOL{}", i + 1))
                .ok_or(HeaderError::GenericError(
                    "Missing TBCOL keyword".to_string(),
                ))?;
            match kw.value {
                KeywordValue::Int(value) => table.tcol.push(value as usize),
                _ => {
                    return Err(HeaderError::GenericError("Invalid TBCOL value".to_string()));
                }
            }
        }

        Ok((HDUData::Table(table), nrows * nrowchars))
    }
}
