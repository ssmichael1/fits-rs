use crate::FITSError;
use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::KeywordValue;
use crate::TDisp;

use crate::utils::*;

use std::error::Error;

mod tform;
use tform::TForm;
use tform::TFormType;

mod tvalue;

pub use tvalue::BinTableValue;

#[derive(Clone, Debug, Default)]
pub struct BinTable {
    pub ncols: usize,
    pub nrows: usize,
    pub theap: usize,
    pub pcount: usize,
    pub rowbytes: usize,
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
    pub tform: Vec<TForm>,
    raw: Vec<u8>,
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
        bintable.rowbytes = get_keyword_int_at_index(header, 3, "NAXIS1")? as usize;
        // Next is NAXIS2
        bintable.nrows = get_keyword_int_at_index(header, 4, "NAXIS2")? as usize;
        // Next is PCOUNT, which is the size of the heap
        bintable.pcount = get_keyword_int_at_index(header, 5, "PCOUNT")? as usize;
        // Next is GCOUNT, must be 1
        check_int_keyword_at_index(header, 6, "GCOUNT", 1)?;
        // Next is TFIELDS
        bintable.ncols = get_keyword_int_at_index(header, 7, "TFIELDS")? as usize;

        // The THEAP keyword tells offset to start of heap
        // It is optional and default is nrowchars*nrows
        bintable.theap = header
            .value_int("THEAP")
            .unwrap_or((bintable.rowbytes * bintable.nrows) as i64)
            as usize;

        bintable.fieldname = Vec::<Option<String>>::with_capacity(bintable.ncols);
        bintable.scale = Vec::<Option<f64>>::with_capacity(bintable.ncols);
        bintable.zero = Vec::<Option<f64>>::with_capacity(bintable.ncols);
        bintable.tdmin = Vec::<Option<f64>>::with_capacity(bintable.ncols);
        bintable.tdmax = Vec::<Option<f64>>::with_capacity(bintable.ncols);
        bintable.tlmin = Vec::<Option<f64>>::with_capacity(bintable.ncols);
        bintable.tlmax = Vec::<Option<f64>>::with_capacity(bintable.ncols);
        bintable.tnull = Vec::<Option<i64>>::with_capacity(bintable.ncols);
        bintable.tform = Vec::<TForm>::with_capacity(bintable.ncols);

        for i in 0..bintable.ncols {
            bintable.tform.push(TForm::from_string(&string_or_err(
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
                .push(header.value_float(&format!("TLMAX{}", i + 1)));

            bintable
                .tdmin
                .push(header.value_float(&format!("TDMIN{}", i + 1)));

            bintable
                .tdmax
                .push(header.value_float(&format!("TDMAX{}", i + 1)));

            bintable
                .tnull
                .push(header.value_int(&format!("TNULL{}", i + 1)));
        }
        let nbytes = bintable.pcount + bintable.theap;

        // Make sure enough bytes are available
        if rawbytes.len() < nbytes {
            return Err(Box::new(crate::FITSError::InvalidDataSize(
                nbytes,
                rawbytes.len(),
            )));
        }
        // Copy raw bytes into the table value
        bintable.raw = rawbytes[..nbytes].to_vec();

        Ok((HDUData::BinTable(bintable), nbytes))
    }

    pub fn at(&self, row: usize, col: usize) -> Result<BinTableValue, Box<dyn Error>> {
        if row >= self.nrows {
            return Err(Box::new(crate::FITSError::InvalidRow(row, self.nrows)));
        }
        if col >= self.ncols {
            return Err(Box::new(crate::FITSError::InvalidColumn(col, self.ncols)));
        }
        let offset = self.rowbytes * row + (0..col).fold(0, |acc, i| acc + self.tform[i].bytes());
        let tform = &self.tform[col];

        let value = match tform.dtype {
            TFormType::Logical => {
                if tform.repeats == 1 {
                    BinTableValue::Logical(self.raw[offset] != 0)
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(self.raw[offset + i] != 0);
                    }
                    BinTableValue::LogicalArr(v)
                }
            }
            TFormType::Bit => {
                if tform.repeats == 1 {
                    BinTableValue::Bit(self.raw[offset] != 0)
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    let nbytes = (tform.repeats + 7) / 8;
                    for i in 0..nbytes {
                        let byte = self.raw[offset + i];
                        for j in 0..8 {
                            v.push((byte & (1 << j)) != 0);
                        }
                    }
                    BinTableValue::BitArr(v)
                }
            }
            TFormType::UnsignedByte => {
                if tform.repeats == 1 {
                    BinTableValue::UnsignedByte(self.raw[offset])
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(self.raw[offset + i]);
                    }
                    BinTableValue::UnsignedByteArr(v)
                }
            }
            TFormType::Int16 => {
                if tform.repeats == 1 {
                    BinTableValue::Int16(i16::from_be_bytes(
                        self.raw[offset..(offset + 2)].try_into().unwrap(),
                    ))
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(i16::from_be_bytes(
                            self.raw[offset + i * 2..(offset + i * 2 + 2)]
                                .try_into()
                                .unwrap(),
                        ));
                    }
                    BinTableValue::Int16Arr(v)
                }
            }
            TFormType::Int32 => {
                if tform.repeats == 1 {
                    BinTableValue::Int32(i32::from_be_bytes(
                        self.raw[offset..(offset + 4)].try_into().unwrap(),
                    ))
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(i32::from_be_bytes(
                            self.raw[offset + i * 4..(offset + i * 4 + 4)]
                                .try_into()
                                .unwrap(),
                        ));
                    }
                    BinTableValue::Int32Arr(v)
                }
            }
            TFormType::Int64 => {
                if tform.repeats == 1 {
                    BinTableValue::Int64(i64::from_be_bytes(
                        self.raw[offset..(offset + 8)].try_into().unwrap(),
                    ))
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(i64::from_be_bytes(
                            self.raw[offset + i * 8..(offset + i * 8 + 8)]
                                .try_into()
                                .unwrap(),
                        ));
                    }
                    BinTableValue::Int64Arr(v)
                }
            }
            TFormType::Char => {
                if tform.repeats == 1 {
                    BinTableValue::Char(self.raw[offset] as char)
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(self.raw[offset + i] as char);
                    }
                    BinTableValue::String(v.into_iter().collect())
                }
            }
            TFormType::Float32 => {
                if tform.repeats == 1 {
                    BinTableValue::Float32(f32::from_be_bytes(
                        self.raw[offset..(offset + 4)].try_into().unwrap(),
                    ))
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(f32::from_be_bytes(
                            self.raw[offset + i * 4..(offset + i * 4 + 4)]
                                .try_into()
                                .unwrap(),
                        ));
                    }
                    BinTableValue::Float32Arr(v)
                }
            }
            TFormType::Float64 => {
                if tform.repeats == 1 {
                    BinTableValue::Float64(f64::from_be_bytes(
                        self.raw[offset..(offset + 8)].try_into().unwrap(),
                    ))
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        v.push(f64::from_be_bytes(
                            self.raw[offset + i * 8..(offset + i * 8 + 8)]
                                .try_into()
                                .unwrap(),
                        ));
                    }
                    BinTableValue::Float64Arr(v)
                }
            }
            TFormType::Complex32 => {
                if tform.repeats == 1 {
                    let real =
                        f32::from_be_bytes(self.raw[offset..(offset + 4)].try_into().unwrap());
                    let imag = f32::from_be_bytes(
                        self.raw[(offset + 4)..(offset + 8)].try_into().unwrap(),
                    );
                    BinTableValue::Complex32((real, imag))
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        let real = f32::from_be_bytes(
                            self.raw[offset + i * 8..(offset + i * 8 + 4)]
                                .try_into()
                                .unwrap(),
                        );
                        let imag = f32::from_be_bytes(
                            self.raw[offset + i * 8 + 4..(offset + i * 8 + 8)]
                                .try_into()
                                .unwrap(),
                        );
                        v.push((real, imag));
                    }
                    BinTableValue::Complex32Arr(v)
                }
            }
            TFormType::Complex64 => {
                if tform.repeats == 1 {
                    let real =
                        f64::from_be_bytes(self.raw[offset..(offset + 8)].try_into().unwrap());
                    let imag = f64::from_be_bytes(
                        self.raw[(offset + 8)..(offset + 16)].try_into().unwrap(),
                    );
                    BinTableValue::Complex64((real, imag))
                } else {
                    let mut v = Vec::with_capacity(tform.repeats);
                    for i in 0..tform.repeats {
                        let real = f64::from_be_bytes(
                            self.raw[offset + i * 16..(offset + i * 16 + 8)]
                                .try_into()
                                .unwrap(),
                        );
                        let imag = f64::from_be_bytes(
                            self.raw[offset + i * 16 + 8..(offset + i * 16 + 16)]
                                .try_into()
                                .unwrap(),
                        );
                        v.push((real, imag));
                    }
                    BinTableValue::Complex64Arr(v)
                }
            }
            _ => {
                return Err(Box::new(crate::FITSError::GenericError(
                    "Array Types Not Yet Implemented".to_string(),
                )))
            }
        };
        Ok(value)
    }
}
