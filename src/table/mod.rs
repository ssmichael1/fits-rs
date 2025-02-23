use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::Keyword;
use crate::KeywordValue;
use crate::TDisp;
use crate::TValue;

use crate::utils::*;

use std::error::Error;

enum TForm {
    Char(usize),
    Int(usize),
    FloatDec(usize, usize),
    FloatE(usize, usize),
    FloatD(usize, usize),
}

/// Structure representing an ASCII table
/// as defined by the FITS Standard,
///
/// # Notes:
/// * See Section 7.2 of the FITS Standard for more information
///
#[derive(Debug, Clone, Default)]
pub struct Table {
    pub nfields: usize,
    pub fieldnames: Vec<String>,
    /// Scale converting from table data to physical units
    pub scale: Vec<Option<f64>>,
    /// Offset converting from table to physical units
    pub zero: Vec<Option<f64>>,
    /// Minimum value of field with valid interpretation
    pub tlmin: Vec<Option<f64>>,
    /// Maximum value of field with valid interpretation
    pub tlmax: Vec<Option<f64>>,
    /// Minimum value of field in column
    pub tdmin: Vec<Option<f64>>,
    /// Maximum value of field in column
    pub tdmax: Vec<Option<f64>>,
    /// The table data
    pub data: Vec<Vec<TValue>>,
    /// The table units
    pub units: Vec<Option<String>>,
}

impl Table {
    fn tform_from_keyword(kw: &Keyword) -> Result<TForm, Box<dyn Error>> {
        if let KeywordValue::String(value) = &kw.value {
            if value.len() < 2 {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid TFORM value".to_string(),
                )));
            }
            let id = value.chars().next().unwrap();
            match id {
                'A' => {
                    let width = value[1..].parse::<usize>()?;
                    Ok(TForm::Char(width))
                }
                'I' => {
                    let width = value[1..].parse::<usize>()?;
                    Ok(TForm::Int(width))
                }
                'F' => {
                    let mut iter = value[1..].split('.');
                    let width = iter.next().unwrap().parse::<usize>()?;
                    let dec = iter.next().unwrap().parse::<usize>()?;
                    Ok(TForm::FloatDec(width, dec))
                }
                'E' => {
                    let mut iter = value[1..].split('.');
                    let width = iter.next().unwrap().parse::<usize>()?;
                    let dec = iter.next().unwrap().parse::<usize>()?;
                    Ok(TForm::FloatE(width, dec))
                }
                'D' => {
                    let mut iter = value[1..].split('.');
                    let width = iter.next().unwrap().parse::<usize>()?;
                    let dec = iter.next().unwrap().parse::<usize>()?;
                    Ok(TForm::FloatD(width, dec))
                }
                _ => Err(Box::new(HeaderError::GenericError(
                    "Invalid TFORM value".to_string(),
                ))),
            }
        } else {
            Err(Box::new(HeaderError::GenericError(
                "Invalid TFORM value".to_string(),
            )))
        }
    }

    pub fn from_bytes(
        header: &Header,
        rawbytes: &[u8],
    ) -> Result<(HDUData, usize), Box<dyn std::error::Error>> {
        // Section 7.2 of the fits standard 4.0 manual
        // Note: this is an objectively awful way to store a table
        // but it is the standard

        let mut table = Box::new(Table::default());
        // Go through required keywords, per the standard
        // After XTENSION is BITPIX, which must be 8
        check_int_keyword_at_index(header, 1, "BITPIX", 8)?;
        // Next is NAXIS, which must be 2
        check_int_keyword_at_index(header, 2, "NAXIS", 2)?;
        // Next is NAXIS1
        let nrowchars = get_keyword_int_at_index(header, 3, "NAXIS1")? as usize;
        // Next is NAXIS2
        let nrows = get_keyword_int_at_index(header, 4, "NAXIS2")? as usize;
        // Next is PCOUNT, which is the size of the heap for table it is zero
        check_int_keyword_at_index(header, 4, "PCOUNT", 0)?;
        // Next is GCOUNT, must be 1
        check_int_keyword_at_index(header, 6, "GCOUNT", 1)?;
        // Next is TFIELDS
        table.nfields = get_keyword_int_at_index(header, 7, "TFIELDS")? as usize;

        let mut tdisp = Vec::<TDisp>::with_capacity(table.nfields);
        let mut tform = Vec::<TForm>::with_capacity(table.nfields);
        let mut tnull = Vec::<Option<String>>::with_capacity(table.nfields);

        let mut tcol = Vec::with_capacity(table.nfields);
        table.tdmin = Vec::with_capacity(table.nfields);
        table.tdmax = Vec::with_capacity(table.nfields);
        table.tlmin = Vec::with_capacity(table.nfields);
        table.tlmax = Vec::with_capacity(table.nfields);
        table.units = Vec::with_capacity(table.nfields);

        for i in 0..table.nfields {
            tcol.push(header.value_int(&format!("TBCOL{}", i + 1)).ok_or(
                HeaderError::GenericError(
                    "Missing or incorrect TBCOL keyword in table".to_string(),
                ),
            )?);

            // TType is name of field ; it is required if TFIELDS is not zero
            table
                .fieldnames
                .push(header.value_string(&format!("TTYPE{}", i + 1)).ok_or(
                    HeaderError::GenericError(
                        "Missing or incorrect TTYPE keyword in table".to_string(),
                    ),
                )?);

            if let Some(kw) = header.find(&format!("TNULL{}", i + 1)) {
                if let KeywordValue::String(value) = &kw.value {
                    tnull.push(Some(value.clone()));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TNULL value".to_string(),
                    )));
                }
            } else {
                tnull.push(None);
            }

            // TForm is the data type; it is required if TFIELDS is not zero
            let kw = header
                .find(&format!("TFORM{}", i + 1))
                .ok_or(HeaderError::GenericError(
                    "Missing TFORM keyword".to_string(),
                ))?;
            tform.push(Table::tform_from_keyword(kw)?);

            if let Some(kw) = header.find(&format!("TDISP{}", i + 1)) {
                tdisp.push(TDisp::from_keyword(kw)?);
            } else {
                tdisp.push(TDisp::None);
            }

            if let Some(kw) = header.find(&format!("TSCAL{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.scale.push(Some(*value));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TSCAL value".to_string(),
                    )));
                }
            } else {
                table.scale.push(None);
            }

            if let Some(kw) = header.find(&format!("TZERO{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.zero.push(Some(*value));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TZERO value".to_string(),
                    )));
                }
            } else {
                table.zero.push(None);
            }

            // Get the units
            if let Some(kw) = header.find(&format!("TUNIT{}", i + 1)) {
                if let KeywordValue::String(value) = &kw.value {
                    table.units.push(Some(value.clone()));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TUNIT value".to_string(),
                    )));
                }
            } else {
                table.units.push(None);
            }

            // Get the minimum physical value in table at this column
            if let Some(kw) = header.find(&format!("TDMIN{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.tdmin.push(Some(*value));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TDMIN value".to_string(),
                    )));
                }
            } else {
                table.tdmin.push(None);
            }

            // Get the maximum physical value in table at this column
            if let Some(kw) = header.find(&format!("TDMAX{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.tdmax.push(Some(*value));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TDMAX value".to_string(),
                    )));
                }
            } else {
                table.tdmax.push(None);
            }

            // Get minimum value of field with valid interpretation
            if let Some(kw) = header.find(&format!("TLMIN{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.tlmin.push(Some(*value));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TLMIN value".to_string(),
                    )));
                }
            } else {
                table.tlmin.push(None);
            }

            //  Get maximum value of field with valid interpretation
            if let Some(kw) = header.find(&format!("TLMAX{}", i + 1)) {
                if let KeywordValue::Float(value) = &kw.value {
                    table.tlmax.push(Some(*value));
                } else {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid TLMAX value".to_string(),
                    )));
                }
            } else {
                table.tlmax.push(None);
            }
        }

        // Make sure data is long enough
        if rawbytes.len() < nrows * nrowchars {
            return Err(Box::new(HeaderError::GenericError(
                "Table data is too short".to_string(),
            )));
        }

        // OK, now actually read in the table data
        table.data = Vec::with_capacity(nrows);
        for i in 0..nrows {
            let mut row = Vec::with_capacity(table.nfields);
            let rowbytes = &rawbytes[i * nrowchars..(i + 1) * nrowchars];
            for j in 0..table.nfields {
                let offset = tcol[j] - 1;
                let width;

                // Check for null value
                if let Some(nullstr) = &tnull[j] {
                    let s = String::from_utf8(rowbytes[offset..offset + nullstr.len()].to_vec())?;
                    if s == *nullstr {
                        row.push(TValue::Null);
                        continue;
                    }
                }

                match &tform[j] {
                    TForm::Char(w) => {
                        width = *w;
                        let s = String::from_utf8(rowbytes[offset..offset + width].to_vec())?;
                        row.push(TValue::String(s));
                    }
                    TForm::Int(w) => {
                        width = *w;
                        let s = String::from_utf8(rowbytes[offset..offset + width].to_vec())?
                            .trim()
                            .to_string();
                        if let Some(scale) = table.scale[j] {
                            if let Some(zero) = table.zero[j] {
                                row.push(TValue::Int((s.parse::<f64>()? * scale + zero) as i64));
                            } else {
                                row.push(TValue::Int((s.parse::<f64>()? * scale) as i64));
                            }
                        } else {
                            row.push(TValue::Int(s.parse::<i64>()?));
                        }
                        row.push(TValue::Int(s.parse::<i64>()?));
                    }
                    TForm::FloatDec(w, _d) => {
                        width = *w;
                        let s = String::from_utf8(rowbytes[offset..offset + width].to_vec())?
                            .trim()
                            .to_string();
                        if let Some(scale) = table.scale[j] {
                            if let Some(zero) = table.zero[j] {
                                row.push(TValue::Float(s.parse::<f64>()? * scale + zero));
                            } else {
                                row.push(TValue::Float(s.parse::<f64>()? * scale));
                            }
                        } else {
                            row.push(TValue::Float(s.parse::<f64>()?));
                        }
                    }
                    TForm::FloatE(w, _d) => {
                        width = *w;
                        let s = String::from_utf8(rowbytes[offset..offset + width].to_vec())?
                            .trim()
                            .to_string();
                        if let Some(scale) = table.scale[j] {
                            if let Some(zero) = table.zero[j] {
                                row.push(TValue::Float(s.parse::<f64>()? * scale + zero));
                            } else {
                                row.push(TValue::Float(s.parse::<f64>()? * scale));
                            }
                        } else {
                            row.push(TValue::Float(s.parse::<f64>()?));
                        }
                    }
                    TForm::FloatD(w, _d) => {
                        width = *w;
                        let s = String::from_utf8(rowbytes[offset..offset + width].to_vec())?
                            .trim()
                            .to_string();
                        let s = str::replace(&s, "D", "E");
                        if let Some(scale) = table.scale[j] {
                            if let Some(zero) = table.zero[j] {
                                row.push(TValue::Float(s.parse::<f64>()? * scale + zero));
                            } else {
                                row.push(TValue::Float(s.parse::<f64>()? * scale));
                            }
                        } else {
                            row.push(TValue::Float(s.parse::<f64>()?));
                        }
                    }
                }
            } // end of iterating over row
            table.data.push(row);
        }

        Ok((HDUData::Table(table), nrows * nrowchars))
    }
}
