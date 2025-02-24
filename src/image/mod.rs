use crate::Bitpix;
use crate::HDUData;
use crate::Header;
use crate::KeywordValue;
use crate::WCS;

use crate::utils::*;

use std::error::Error;

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
    /// # Arguments:
    ///
    /// * `header` - Header information for the image
    /// * `rawbytes` - Raw bytes starting from "data" portion of HDU
    ///
    /// # Note:
    /// This is most likely called by the `HDU` struct and not directly
    /// 
    /// # Returns:
    ///
    /// * `HDUData` - Image data
    /// * `usize` - Number of bytes consumed
    ///
    pub(crate) fn from_bytes(
        header: &Header,
        rawbytes: &[u8],
    ) -> Result<(HDUData, usize), Box<dyn std::error::Error>> {
        let mut image = HDUData::None;

        let bitpixval = get_keyword_int_at_index(header, 1, "BITPIX")?;
        let bitpix = Bitpix::from_i64(bitpixval)?;
        let naxis = get_keyword_int_at_index(header, 2, "NAXIS")? as usize;
        let axes = (0..naxis)
            .map(|x| -> Result<usize, Box<dyn Error>> {
                let ax = get_keyword_int_at_index(header, x + 3, &format!("NAXIS{}", x + 1))?;
                Ok(ax as usize)
            })
            .collect::<Result<Vec<usize>, Box<dyn std::error::Error>>>()?;

        let mut pcount = 0;
        let mut gcount = 1;
        if KeywordValue::String("IMAGE".to_string()) == header[0].value {
            // loog for PCOUNT and GCOUNT keywords
            pcount = get_keyword_int_at_index(header, 3 + naxis, "PCOUNT")? as usize;
            gcount = get_keyword_int_at_index(header, 4 + naxis, "GCOUNT")? as usize;
        }
        let npixels = match axes.is_empty() {
            true => 0,
            _ => axes.iter().product(),
        } as usize;
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

    /// Access raw pixels in native format. This must be explicitly set and is
    /// based upon the pixel type
    ///
    /// # Note: endian conversion has alread been performed if needed on data ingest
    ///
    /// # Casting based upon Bitpix  values
    /// * `Bitpix::Int8`    : u8
    /// * `Bitpix::Int16`   : u16
    /// * `Bitpix::Int32`   : u32
    /// * `Bitpix::Int64`   : u64
    /// * `Bitpix::Float32` : f32
    /// * `Bitpix::Float64` : f64
    /// 
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
