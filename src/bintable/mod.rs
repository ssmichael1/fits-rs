use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::Keyword;
use crate::KeywordValue;
use crate::TDisp;

use std::error::Error;

#[derive(Clone, Debug, Default)]
pub struct BinTable {
    pub nfields: usize,
    pub fieldname: Vec<String>,
    pub scale: Vec<Option<f64>>,
    pub zero: Vec<Option<f64>>,
    pub tdisp: Vec<TDisp>,
}

impl BinTable {
    pub fn from_bytes(
        header: &Header,
        rawbytes: &[u8],
    ) -> Result<(HDUData, usize), Box<dyn Error>> {
        let mut bintable = Box::new(BinTable::default());

        // Go through required keywords, per
        // Table 17 of the FITS Standard
        let kwbitpix = header
            .get(1)
            .ok_or(HeaderError::MissingKeyword("BITPIX".to_string()))?;
        if kwbitpix.name != "BITPIX" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwbitpix.name.clone(),
                1,
            )));
        }
        if let KeywordValue::Int(value) = &kwbitpix.value {
            if *value != 8 {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid BITPIX value".to_string(),
                )));
            }
        } else {
            return Err(Box::new(HeaderError::UnexpectedValueType(
                kwbitpix.name.clone(),
            )));
        }

        let kwnaxis = header
            .get(2)
            .ok_or(HeaderError::MissingKeyword("NAXIS".to_string()))?;
        if kwnaxis.name != "NAXIS" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwnaxis.name.clone(),
                2,
            )));
        }
        if let KeywordValue::Int(value) = &kwnaxis.value {
            if *value != 2 {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid NAXIS value".to_string(),
                )));
            }
        } else {
            return Err(Box::new(HeaderError::UnexpectedValueType(
                kwnaxis.name.clone(),
            )));
        }

        let kwaxis1 = header
            .get(3)
            .ok_or(HeaderError::MissingKeyword("NAXIS1".to_string()))?;
        if kwaxis1.name != "NAXIS1" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwaxis1.name.clone(),
                3,
            )));
        }
        let nrowchars = match &kwaxis1.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(Box::new(HeaderError::UnexpectedValueType(
                    kwaxis1.name.clone(),
                )))
            }
        };

        let kwaxis2 = header
            .get(4)
            .ok_or(HeaderError::MissingKeyword("NAXIS2".to_string()))?;
        if kwaxis2.name != "NAXIS2" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwaxis2.name.clone(),
                4,
            )));
        }
        let nrows = match &kwaxis2.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(Box::new(HeaderError::UnexpectedValueType(
                    kwaxis2.name.clone(),
                )));
            }
        };

        let kwpcount = header
            .get(5)
            .ok_or(HeaderError::MissingKeyword("PCOUNT".to_string()))?;
        if kwpcount.name != "PCOUNT" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwpcount.name.clone(),
                5,
            )));
        }
        let pcount = match &kwpcount.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(Box::new(HeaderError::UnexpectedValueType(
                    kwpcount.name.clone(),
                )));
            }
        };

        let kwgcount = header
            .get(6)
            .ok_or(HeaderError::MissingKeyword("GCOUNT".to_string()))?;
        if kwgcount.name != "GCOUNT" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwgcount.name.clone(),
                6,
            )));
        }
        if let KeywordValue::Int(value) = &kwgcount.value {
            if *value != 1 {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid GCOUNT value".to_string(),
                )));
            }
        } else {
            return Err(Box::new(HeaderError::UnexpectedValueType(
                kwgcount.name.clone(),
            )));
        }

        let kwtfields = header
            .get(7)
            .ok_or(HeaderError::MissingKeyword("TFIELDS".to_string()))?;
        if kwtfields.name != "TFIELDS" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwtfields.name.clone(),
                7,
            )));
        }
        bintable.nfields = match &kwtfields.value {
            KeywordValue::Int(value) => *value as usize,
            _ => {
                return Err(Box::new(HeaderError::UnexpectedValueType(
                    kwtfields.name.clone(),
                )));
            }
        };

        Ok((HDUData::BinTable(bintable), 0))
    }
}
