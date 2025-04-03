use crate::Keyword;
use crate::KeywordValue;

use anyhow::{anyhow, Context, Result};

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

fn wmfromstr(s: &str) -> Result<(usize, usize)> {
    let mut iter = s.split('.');
    let w = iter.next().context("Invalid TDISP value")?;
    let m = {
        if let Some(mstr) = iter.next() {
            mstr.parse::<usize>()?
        } else {
            0
        }
    };
    Ok((w.parse()?, m))
}

fn wdefromstr(s: &str) -> Result<(usize, usize, usize)> {
    let mut iter = s.split('.');
    let w = iter.next().context("Invalid TDISP value")?;
    let w: usize = w.parse()?;
    let mut d: usize = 0;
    let mut e: usize = 0;
    if let Some(dstr) = iter.next() {
        let mut iter2 = dstr.split('E');
        let dstr = iter2.next().context("Invalid TDISP value")?;
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
    pub fn from_keyword(kw: &Keyword) -> Result<TDisp> {
        if let KeywordValue::String(value) = &kw.value {
            let disp = value.chars().next().context("Invalid TDISP value")?;
            let fstr = value.chars().skip(1).collect::<String>();
            match disp {
                'A' => Ok(TDisp::Char(fstr.parse().context("Invalid TDISP value")?)),
                'L' => Ok(TDisp::Logical(fstr.parse().context("Invalid TDISP value")?)),
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
                _ => Err(anyhow!("Invalid TDISP value: {}", value)),
            }
        } else {
            Err(anyhow!(
                "Invalid TDISP value: expected String, got {:?}",
                kw.value
            ))
        }
    }
}
