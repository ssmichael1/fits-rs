use crate::Keyword;

use anyhow::{anyhow, Context, Result};

#[derive(Clone, Debug)]
pub struct FITSBlock(pub [Keyword; 36]);

impl FITSBlock {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 2880 {
            return Err(anyhow!("Invalid FITS block size: {}", bytes.len()));
        }

        Ok(FITSBlock(
            (0..36)
                .map(|i| {
                    let record = &bytes[i * 80..(i + 1) * 80];
                    Keyword::from_bytes(record)
                })
                .collect::<anyhow::Result<Vec<_>>>()
                .context("Failed to parse headers in FITS block")?
                .try_into()
                .map_err(|_| anyhow!("Failed to convert to FITSBlock"))?,
        ))
    }
}
