use crate::header::Header;
use crate::image::Image;
use crate::Bitpix;
use crate::HDUData;

use anyhow::{anyhow, Result};

pub enum CompressionType {
    Rice,
    Gzip1,
    Gzip2,
    #[allow(clippy::upper_case_acronyms)]
    PLIO,
    HCompress,
    Uncompressed,
}

impl TryFrom<String> for CompressionType {
    type Error = anyhow::Error;

    /// Convert from a string to a CompressionType
    /// See Table 36 of the FITS Standard for more information
    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "NOCOMPRESS" => Ok(CompressionType::Uncompressed),
            "RICE_1" => Ok(CompressionType::Rice),
            "GZIP_1" => Ok(CompressionType::Gzip1),
            "GZIP_2" => Ok(CompressionType::Gzip2),
            "PLIO_1" => Ok(CompressionType::PLIO),
            "HCOMPRESS_1" => Ok(CompressionType::HCompress),
            _ => Err(anyhow!("Invalid compression type: {}", value)),
        }
    }
}

/// Convert a binary table containing a compressed image to
/// an uncompressed image
///
/// # Authors note:
///
/// FITS file compression is implemented in a shitty shitty way ... why would
/// anyone ever do this?!
///
pub fn bintable2image(header: &Header, _bintable: &HDUData) -> Result<Image> {
    // Mandatory CMPTYPE keyword
    // Section 10.1.1 of standard
    let cmptype = header
        .find("CMPTYPE")
        .ok_or_else(|| anyhow!("Missing CMPTYPE keyword"))?
        .get_string()?;
    let _compression_type = CompressionType::try_from(cmptype)?;
    let zbitpix = header
        .find("ZBITPIX")
        .ok_or_else(|| anyhow!("Missing ZBITPIX keyword"))?
        .get_int()?;
    let _bitpix = Bitpix::from_i64(zbitpix)?;
    let naxis = header
        .find("ZNAXIS")
        .ok_or_else(|| anyhow!("Missing ZNAXIS keyword"))?
        .get_int()?;
    let mut axes = Vec::new();
    let mut tile = Vec::new();
    for i in 1..=naxis {
        let naxis = header
            .find(&format!("ZNAXIS{}", i))
            .ok_or_else(|| anyhow!("Missing ZNAXIS{} keyword", i))?
            .get_int()?;
        axes.push(naxis);
        tile.push({
            if let Some(ztile) = header.find(&format!("ZTILE{}", i)) {
                ztile.get_int()?
            } else if i == 1 {
                naxis
            } else {
                1
            }
        });
    }

    Err(anyhow!("Compression not yet supported"))
}
