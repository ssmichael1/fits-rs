use crate::Bitpix;
use crate::HDUData;
use crate::Header;
use crate::HeaderError;
use crate::KeywordValue;
use crate::WCS;

/// Represent image data as described in a FITS file
///
/// # This includes:
/// * Raw pixel data in image
/// * Axes definitions
/// * World Coordinate System transform(s) information
///
/// # Note
///
/// * Derived from Version 4 of FITS standard
#[derive(Clone, Debug)]
pub struct Image {
    pub pixeltype: Bitpix,
    pub axes: Vec<usize>,
    pub rawbytes: Vec<u8>,
    pub wcs: Option<WCS>,
    pub gcount: usize,
    pub pcount: usize,
}

impl Image {
    /// Number of dimensions
    pub fn ndims(&self) -> usize {
        self.axes.len()
    }

    /// Construct an image from raw bytes from the file
    ///
    /// Arguments:
    ///
    /// * `header` - Header information for the image
    /// * `rawbytes` - Raw bytes starting from "data" portion of HDU
    ///
    /// Returns:
    ///
    /// * `HDUData` - Image data
    /// * `usize` - Number of bytes consumed
    ///
    pub(crate) fn from_bytes(
        header: &Header,
        rawbytes: &[u8],
    ) -> Result<(HDUData, usize), Box<dyn std::error::Error>> {
        let mut image = HDUData::None;

        let kwbitpix = header
            .get(1)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwbitpix.name != "BITPIX" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwbitpix.name.clone(),
                1,
            )));
        }
        let bitpix = match &kwbitpix.value {
            KeywordValue::Int(value) => Bitpix::from_i64(*value)?,
            _ => {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid BITPIX value".to_string(),
                )))
            }
        };
        let kwaxes = header
            .get(2)
            .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
        if kwaxes.name != "NAXIS" {
            return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                kwaxes.name.clone(),
                2,
            )));
        }
        let naxis = match &kwaxes.value {
            KeywordValue::Int(value) => *value as u16,
            _ => {
                return Err(Box::new(HeaderError::GenericError(
                    "Invalid NAXIS value".to_string(),
                )))
            }
        };
        let mut axes = Vec::with_capacity(naxis as usize);
        for i in 0..naxis {
            let kwaxis = header
                .get(3 + i as usize)
                .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
            if kwaxis.name != format!("NAXIS{}", i + 1) {
                return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                    kwaxis.name.clone(),
                    3 + i as usize,
                )));
            }
            let axis = match &kwaxis.value {
                KeywordValue::Int(value) => *value as usize,
                _ => {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid NAXIS value".to_string(),
                    )))
                }
            };
            axes.push(axis);
        }
        let mut pcount = 0;
        let mut gcount = 1;
        if KeywordValue::String("IMAGE".to_string()) == header[0].value {
            // loog for PCOUNT and GCOUNT keywords

            let kwidx = 4 + naxis as usize;
            let kwpcount = header
                .get(kwidx)
                .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
            if kwpcount.name != "PCOUNT" {
                return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                    kwpcount.name.clone(),
                    kwidx,
                )));
            }
            match &kwpcount.value {
                KeywordValue::Int(value) => pcount = *value as usize,
                _ => {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid PCOUNT value".to_string(),
                    )))
                }
            }
            let kwgcount = header
                .get(kwidx + 1)
                .ok_or(HeaderError::GenericError("not enough keywords".to_string()))?;
            if kwgcount.name != "GCOUNT" {
                return Err(Box::new(HeaderError::InvalidKeywordPlacement(
                    kwgcount.name.clone(),
                    kwidx + 1,
                )));
            }
            match &kwgcount.value {
                KeywordValue::Int(value) => gcount = *value as usize,
                _ => {
                    return Err(Box::new(HeaderError::GenericError(
                        "Invalid GCOUNT value".to_string(),
                    )))
                }
            }
        }

        let npixels = axes.iter().product::<usize>();
        let nbytes = npixels * bitpix.size();
        if nbytes > 0 {
            // Extract raw bytes of image, but make sure they match large endian format
            // for fast data retreival later
            let imgrawbytes: Vec<u8> = match bitpix {
                Bitpix::Int8 => rawbytes[0..nbytes].to_vec(),
                Bitpix::Int16 => {
                    // Convert raw bytes to u16 taking as big endian
                    bytemuck::cast_slice(
                        &rawbytes[0..nbytes]
                            .chunks_exact(2)
                            .map(|x| u16::from_be_bytes(x.try_into().unwrap()))
                            .collect::<Vec<u16>>(),
                    )
                    .to_vec()
                }
                Bitpix::Int32 => {
                    // Convert raw bytes to u32 taking as big endian
                    bytemuck::cast_slice(
                        &rawbytes[0..nbytes]
                            .chunks_exact(4)
                            .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
                            .collect::<Vec<u32>>(),
                    )
                    .to_vec()
                }
                Bitpix::Int64 => {
                    // Convert raw bytes to u64 taking as big endian
                    bytemuck::cast_slice(
                        &rawbytes[0..nbytes]
                            .chunks_exact(8)
                            .map(|x| u64::from_be_bytes(x.try_into().unwrap()))
                            .collect::<Vec<u64>>(),
                    )
                    .to_vec()
                }
                Bitpix::Float32 => {
                    // Convert raw bytes to f32 taking as big endian
                    bytemuck::cast_slice(
                        &rawbytes[0..nbytes]
                            .chunks_exact(4)
                            .map(|x| f32::from_be_bytes(x.try_into().unwrap()))
                            .collect::<Vec<f32>>(),
                    )
                    .to_vec()
                }
                Bitpix::Float64 => {
                    // Convert raw bytes to f64 taking as big endian
                    bytemuck::cast_slice(
                        &rawbytes[0..nbytes]
                            .chunks_exact(8)
                            .map(|x| f64::from_be_bytes(x.try_into().unwrap()))
                            .collect::<Vec<f64>>(),
                    )
                    .to_vec()
                }
            };
            image = HDUData::Image(Box::new(Image {
                pixeltype: bitpix,
                axes,
                rawbytes: imgrawbytes,
                wcs: crate::WCS::from_header(header)?,
                gcount,
                pcount,
            }))
        }

        Ok((image, nbytes))
    }

    /// Access raw pixels in native format ...
    /// This must be explicitly set and is
    /// based upon the pixel type
    ///
    /// # Note: endian conversion has already
    ///   been performed if needed on data ingest
    ///
    /// # Casting based upon Bitpix  values
    /// * `Bitpix::Int8`    : u8
    /// * `Bitpix::Int16`   : u16
    /// * `Bitpix::Int32`   : u32
    /// * `Bitpix::Int64`   : u64
    /// * `Bitpix::Float32` : f32
    /// * `Bitpix::Float64` : f64
    pub fn pixels<T>(&self) -> &[T]
    where
        T: bytemuck::Pod,
    {
        bytemuck::cast_slice(&self.rawbytes)
    }

    /// Get pixel value at a given location
    ///
    /// # Casting based upon Bitpix  values
    /// * `Bitpix::Int8`    : u8
    /// * `Bitpix::Int16`   : u16
    /// * `Bitpix::Int32`   : u32
    /// * `Bitpix::Int64`   : u64
    /// * `Bitpix::Float32` : f32
    /// * `Bitpix::Float64` : f64
    ///
    pub fn at<T>(&self, loc: &[usize]) -> T
    where
        T: bytemuck::Pod,
    {
        let bitsize = self.pixeltype.size();
        let mut offmult = 1;
        let mut offset = 0;
        // The first index increments most rapidly
        for (ix, &loc_val) in loc.iter().enumerate() {
            offset += offmult * loc_val;
            offmult *= self.axes[ix];
        }
        bytemuck::cast_slice(&self.rawbytes[offset..(offset + bitsize)])[0]
    }
}
