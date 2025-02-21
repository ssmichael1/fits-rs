use crate::HeaderError;
use crate::Keyword;

#[derive(Clone, Debug)]
pub struct FITSBlock(pub [Keyword; 36]);

impl FITSBlock {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if bytes.len() != 2880 {
            return Err(Box::new(HeaderError::InvalidHeader));
        }

        Ok(FITSBlock(
            (0..36)
                .map(|i| {
                    let record = &bytes[i * 80..(i + 1) * 80];
                    Keyword::new(record)
                })
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|_| HeaderError::InvalidHeader)?,
        ))
    }
}
