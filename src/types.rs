#[derive(Debug, Clone)]
pub enum HDUData {
    None,
    Table(Box<crate::Table>),
    Image(Box<crate::Image>),
}

/// Bit Pix Types
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
    pub fn from_i64(value: i64) -> Result<Self, Box<dyn std::error::Error>> {
        match value {
            8 => Ok(Bitpix::Int8),
            16 => Ok(Bitpix::Int16),
            32 => Ok(Bitpix::Int32),
            64 => Ok(Bitpix::Int64),
            -32 => Ok(Bitpix::Float32),
            -64 => Ok(Bitpix::Float64),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid BITPIX value",
            ))),
        }
    }

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
