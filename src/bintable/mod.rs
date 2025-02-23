use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::KeywordValue;
use crate::TDisp;

use crate::utils::*;

use std::error::Error;

mod tform;
use tform::TForm;

#[derive(Clone, Debug, Default)]
pub struct BinTable {
    pub nfields: usize,
    pub fieldname: Vec<Option<String>>,
    pub scale: Vec<Option<f64>>,
    pub zero: Vec<Option<f64>>,
    pub tdisp: Vec<TDisp>,
    pub units: Vec<Option<String>>,
    pub tdmin: Vec<Option<f64>>,
    pub tdmax: Vec<Option<f64>>,
    pub tlmin: Vec<Option<f64>>,
    pub tlmax: Vec<Option<f64>>,
    pub tnull: Vec<Option<i64>>,
}

fn string_or_err(header: &Header, kw: &str) -> Result<String, Box<dyn Error>> {
    if let Some(kw) = header.value(kw) {
        if let KeywordValue::String(s) = kw {
            Ok(s.clone())
        } else {
            Err(Box::new(HeaderError::UnexpectedValueType(kw.to_string())))
        }
    } else {
        Err(Box::new(HeaderError::MissingKeyword(kw.to_string())))
    }
}

impl BinTable {
    pub fn from_bytes(
        header: &Header,
        rawbytes: &[u8],
    ) -> Result<(HDUData, usize), Box<dyn Error>> {
        let mut bintable = Box::new(BinTable::default());

        // Go through required keywords, per the standard
        // After XTENSION is BITPIX, which must be 8
        check_int_keyword_at_index(header, 1, "BITPIX", 8)?;
        // Next is NAXIS, which must be 2
        check_int_keyword_at_index(header, 2, "NAXIS", 2)?;
        // Next is NAXIS1
        let nrowchars = get_keyword_int_at_index(header, 3, "NAXIS1")?;
        // Next is NAXIS2
        let nrows = get_keyword_int_at_index(header, 4, "NAXIS2")?;
        // Next is PCOUNT, which is the size of the heap
        let pcount = get_keyword_int_at_index(header, 5, "PCOUNT")?;
        // Next is GCOUNT, must be 1
        check_int_keyword_at_index(header, 6, "GCOUNT", 1)?;
        // Next is TFIELDS
        let nfields = get_keyword_int_at_index(header, 7, "TFIELDS")? as usize;

        // The THEAP keyword tells offset to start of heap
        // It is optional and default is nrowchars*nrows
        let theap = header.value_int("THEAP").unwrap_or(nrowchars * nrows);

        let mut tformarr = Vec::<TForm>::with_capacity(nfields);
        bintable.fieldname = Vec::<Option<String>>::with_capacity(nfields);
        bintable.scale = Vec::<Option<f64>>::with_capacity(nfields);
        bintable.zero = Vec::<Option<f64>>::with_capacity(nfields);
        bintable.tdmin = Vec::<Option<f64>>::with_capacity(nfields);
        bintable.tdmax = Vec::<Option<f64>>::with_capacity(nfields);
        bintable.tlmin = Vec::<Option<f64>>::with_capacity(nfields);
        bintable.tlmax = Vec::<Option<f64>>::with_capacity(nfields);
        bintable.tnull = Vec::<Option<i64>>::with_capacity(nfields);

        println!("going thruogh fields");
        for i in 0..nfields {
            tformarr.push(TForm::from_string(&string_or_err(
                header,
                format!("TFORM{}", i + 1).as_str(),
            )?)?);

            bintable
                .fieldname
                .push(header.value_string(&format!("TTYPE{}", i + 1)));

            bintable
                .scale
                .push(header.value_float(&format!("TSCAL{}", i + 1)));

            bintable
                .zero
                .push(header.value_float(&format!("TZERO{}", i + 1)));

            bintable
                .units
                .push(header.value_string(&format!("TUNIT{}", i + 1)));

            bintable
                .tlmin
                .push(header.value_float(&format!("TLMIN{}", i + 1)));

            bintable
                .tlmax
                .push(header.value_float(&format!("TLMAXP{}", i + 1)));

            bintable
                .tdmin
                .push(header.value_float(&format!("TDMINP{}", i + 1)));

            bintable
                .tdmax
                .push(header.value_float(&format!("TDMAX{}", i + 1)));

            bintable
                .tnull
                .push(header.value_int(&format!("TNULL{}", i + 1)));
        }

        Ok((
            HDUData::BinTable(bintable),
            (theap + pcount).try_into().unwrap(),
        ))
    }
}
