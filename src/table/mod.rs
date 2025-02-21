use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::KeywordValue;

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub nfields: usize,
    pub tcol: Vec<usize>,
    pub fieldnames: Vec<String>,
    pub scale: Vec<Option<f64>>,
    pub zero: Vec<Option<f64>>,
}

enum TValue {
    Char(String),
    Int(i64),
    Float(f64),
}

enum TDisp {
    Char(usize),
    Int(usize, usize),
    Bin(usize, usize),
    Oct(usize, usize),
    Hex(usize, usize),
    Float(usize, usize),
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

        let mut tdisp = Vec::<TDisp>::with_capacity(table.nfields);

        table.tcol = Vec::with_capacity(table.nfields);
        for i in 0..table.nfields {
            let kw = header
                .find(&format!("TBCOL{}", i + 1))
                .ok_or(HeaderError::GenericError(
                    "Missing TBCOL keyword".to_string(),
                ))?;
            if let KeywordValue::Int(val) = kw.value {
                table.tcol.push(val as usize);
            } else {
                return Err(HeaderError::GenericError("Invalid TBCOL value".to_string()));
            }

            let kw = header
                .find(&format!("TTYPE{}", i + 1))
                .ok_or(HeaderError::GenericError(
                    "Missing TTYPE keyword".to_string(),
                ))?;
            if let KeywordValue::String(value) = &kw.value {
                table.fieldnames.push(value.clone());
            } else {
                return Err(HeaderError::GenericError("Invalid TTYPE value".to_string()));
            }

            if let Some(kw) = header.find(&format!("TSCAL{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.scale.push(Some(*value));
                } else {
                    return Err(HeaderError::GenericError("Invalid TSCAL value".to_string()));
                }
            } else {
                table.scale.push(None);
            }

            if let Some(kw) = header.find(&format!("TZERO{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.zero.push(Some(*value));
                } else {
                    return Err(HeaderError::GenericError("Invalid TZERO value".to_string()));
                }
            } else {
                table.zero.push(None);
            }

            if let Some(kw) = header.find(&format!("TDISP{}", i + 1)) {
                if let KeywordValue::String(value) = &kw.value {
                    let mut iter = value.split(|c| c == '.');
                    let width = iter.next().unwrap().parse::<usize>().unwrap();
                    let disp = iter.next().unwrap();
                    tdisp.push(match disp {
                        "A" => TDisp::Char(width),
                        "I" => TDisp::Int(width, 10),
                        "B" => TDisp::Bin(width, 2),
                        "O" => TDisp::Oct(width, 8),
                        "Z" => TDisp::Hex(width, 16),
                        "F" => TDisp::Float(width, 10),
                        _ => {
                            return Err(HeaderError::GenericError(
                                "Invalid TDISP value".to_string(),
                            ));
                        }
                    });
                } else {
                    return Err(HeaderError::GenericError("Invalid TDISP value".to_string()));
                }
            }
        }
        Ok((HDUData::Table(table), nrows * nrowchars))
    }
}
