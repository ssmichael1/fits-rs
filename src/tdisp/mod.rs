use crate::errors::FITSError;
use crate::Keyword;
use crate::KeywordValue;

use std::error::Error;

#[derive(Clone, Debug)]
pub enum TDisp {
    None,
    Char(usize),
    Logical(usize),
    Int(usize, usize),
    Bin(usize, usize),
    Oct(usize, usize),
    Hex(usize, usize),
    Float(usize, usize),
    FloatGen(usize, usize, usize),
    FloatExp(usize, usize, usize),
}

fn wmfromstr(s: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let mut iter = s.split('.');
    let w = iter
        .next()
        .ok_or_else(|| Box::new(FITSError::GenericError("Invalid TDISP value".to_string())))?;
    let m = {
        if let Some(mstr) = iter.next() {
            mstr.parse::<usize>()?
        } else {
            0
        }
    };
    Ok((w.parse()?, m))
}

fn wdefromstr(s: &str) -> Result<(usize, usize, usize), Box<dyn Error>> {
    let mut iter = s.split('.');
    let w = iter
        .next()
        .ok_or_else(|| Box::new(FITSError::GenericError("Invalid TDISP value".to_string())))?;
    let w: usize = w.parse()?;
    let mut d: usize = 0;
    let mut e: usize = 0;
    if let Some(dstr) = iter.next() {
        let mut iter2 = dstr.split('E');
        let dstr = iter2.next().ok_or_else(|| {
            Box::new(FITSError::GenericError("Invalid TDISP value".to_string()))
        })?;
        d = dstr.parse::<usize>()?;
        if let Some(estr) = iter2.next() {
            e = estr.parse::<usize>()?;
        }
    }
    Ok((w, d, e))
}

impl TDisp {
    /// Parse a TDISP keyword
    /// See Table 16 of the FITS Standard
    pub fn from_keyword(kw: &Keyword) -> Result<TDisp, Box<dyn Error>> {
        if let KeywordValue::String(value) = &kw.value {
            let disp = value
                .chars()
                .next()
                .ok_or(FITSError::GenericError("Invalid TDISP value".to_string()))?;
            let fstr = value.chars().skip(1).collect::<String>();
            match disp {
                'A' => Ok(TDisp::Char(fstr.parse().map_err(|_| {
                    Box::new(FITSError::GenericError("Invalid TDISP value".to_string()))
                })?)),
                'L' => Ok(TDisp::Logical(fstr.parse().map_err(|_| {
                    Box::new(FITSError::GenericError("Invalid TDISP value".to_string()))
                })?)),
                'I' => {
                    let (w, m) = wmfromstr(&fstr)?;
                    Ok(TDisp::Int(w, m))
                }
                'B' => {
                    let (w, m) = wmfromstr(&fstr)?;
                    Ok(TDisp::Bin(w, m))
                }
                'O' => {
                    let (w, m) = wmfromstr(&fstr)?;
                    Ok(TDisp::Oct(w, m))
                }
                'Z' => {
                    let (w, m) = wmfromstr(&fstr)?;
                    Ok(TDisp::Hex(w, m))
                }
                'F' => {
                    let (w, d) = wmfromstr(&fstr)?;
                    Ok(TDisp::Float(w, d))
                }
                'G' => {
                    let (w, d, e) = wdefromstr(&fstr)?;
                    Ok(TDisp::FloatGen(w, d, e))
                }
                'D' => {
                    let (w, d, e) = wdefromstr(&fstr)?;
                    Ok(TDisp::FloatExp(w, d, e))
                }
                'E' => {
                    if fstr.starts_with('N') || fstr.starts_with('S') {
                        let fstr = fstr.chars().skip(1).collect::<String>();
                        let (w, d) = wmfromstr(&fstr)?;
                        Ok(TDisp::FloatExp(w, d, 3))
                    } else {
                        let (w, d, e) = wdefromstr(&fstr)?;
                        Ok(TDisp::FloatExp(w, d, e))
                    }
                }
                _ => Err(Box::new(FITSError::GenericError(
                    "Invalid TDISP value".to_string(),
                ))),
            }
        } else {
            Err(Box::new(FITSError::GenericError(
                "Invalid TDISP value".to_string(),
            )))
        }
    }
}
