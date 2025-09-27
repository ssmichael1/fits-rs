use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum HDUData {
    None,
    Table(crate::Table),
    Image(crate::Image),
    BinTable(crate::BinTable),
}

#[derive(Debug, Clone)]
pub enum TValue {
    String(String),
    Int(i64),
    Float(f64),
    Null,
}

pub enum BinTVaue {}

/// "BITPIX" is a keyword in a a FITS header that describes
/// the storage of raw pixels in the data section of the HDU
///
/// See Section 4.4.1 of the FITS Standard for more information
///
/// - [FITS Standard](https://fits.gsfc.nasa.gov/standard40/fits_standard40aa-le.pdf)
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bitpix {
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
}

impl Bitpix {
    /// Get from raw integer values
    /// See Table 8 of the FITS Standard for more information
    pub fn from_i64(value: i64) -> Result<Self> {
        match value {
            8 => Ok(Bitpix::Int8),
            16 => Ok(Bitpix::Int16),
            32 => Ok(Bitpix::Int32),
            64 => Ok(Bitpix::Int64),
            -32 => Ok(Bitpix::Float32),
            -64 => Ok(Bitpix::Float64),
            _ => Err(anyhow!(
                "Invalid BITPIX value: {}. Must be one of 8, 16, 32, 64, -32, -64",
                value
            )),
        }
    }

    /// Convert to raw integer values
    /// See Table 8 of the FITS Standard for more information
    pub fn to_i64(&self) -> i64 {
        match self {
            Bitpix::Int8 => 8,
            Bitpix::Int16 => 16,
            Bitpix::Int32 => 32,
            Bitpix::Int64 => 64,
            Bitpix::Float32 => -32,
            Bitpix::Float64 => -64,
        }
    }

    /// Get the size of the data type in bytes
    pub fn size(&self) -> usize {
        match self {
            Bitpix::Int8 => 1,
            Bitpix::Int16 => 2,
            Bitpix::Int32 => 4,
            Bitpix::Int64 => 8,
            Bitpix::Float32 => 4,
            Bitpix::Float64 => 8,
        }
    }
}
