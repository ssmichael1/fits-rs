use crate::HDU;

use std::io::Read;

/// FITS File Structure
///
/// # Description:
///
/// The NASA FITS file format is a standard format for astronomical data.  It is a
/// binary format that consists of a series of Header and Data Units (HDUs).  Each
/// HDU consists of a header and data section.  The header is a series of 80 byte
/// records that contain keyword/value pairs.  The data section contains the actual
/// data.  The data can be in a variety of formats including images, tables, and
/// binary tables.
///
/// This module provides a structure for reading and writing FITS files, and inerpreting
/// the contained binary data.
///
///
/// # References:
/// - [FITS File Format](https://fits.gsfc.nasa.gov/fits_primer.html)
/// - [FITS Standard](https://fits.gsfc.nasa.gov/standard40/fits_standard40aa-le.pdf)
/// - [FITS Header Keywords](https://fits.gsfc.nasa.gov/fits_dictionary.html)
///
///
/// # Examples:
///
/// * The following example reads a FITS file and prints the contents of the file:
/// ```
/// use fits_rs::FITS;
/// let fits = FITS::from_file("samp/WFPC2u5780205r_c0fx.fits");
///     match fits {
///         Ok(fits) => {
///         println!("{}", fits);
///     }
///     Err(e) => {
///         println!("Error: {}", e);
///         panic!("Error reading FITS file");
///     }
/// }
/// ```
///
/// * The following example reads a FITS file and prints the contents of the first HDU:
/// ```
/// use fits_rs::FITS;
/// let fits = FITS::from_file("samp/WFPC2u5780205r_c0fx.fits");
///     match fits {
///         Ok(fits) => {
///         println!("{}", fits[0]);
///     }
///     Err(e) => {
///         println!("Error: {}", e);
///         panic!("Error reading FITS file");
///     }
/// }
/// ```
///
/// * The following example accesses an image in the first (primary) HDU
/// * and some of the associated image fields
///
/// ```
/// use fits_rs::FITS;
/// use fits_rs::HDUData;
/// let fits = FITS::from_file("samp/WFPC2u5780205r_c0fx.fits");
///     match fits {
///         Ok(fits) => {
///         if let HDUData::Image(im) = &fits[0].data {
///             println!("Image shape: {:?}", im.axes);
///             println!("Image pixel type: {:?}", im.pixeltype);
///             println!("Image WCS: {:?}", im.wcs);
///         }
///     }
///     Err(e) => {
///         println!("Error: {}", e);
///         panic!("Error reading FITS file");
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct FITS {
    hdus: Vec<HDU>,
}

impl std::fmt::Display for FITS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for record in &self.hdus {
            write!(f, "\n{}", record)?;
        }
        Ok(())
    }
}

impl FITS {

    /// Return the HDU at the given index
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the HDU to return
    ///
    /// # Returns
    ///
    /// A reference to the HDU if it exists, otherwise an error
    ///
    pub fn at(&self, index: usize) -> Result<&HDU, Box<dyn std::error::Error>> {
        if index < self.hdus.len() {
            Ok(&self.hdus[index])
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Index out of bounds",
            )))
        }
    }

    /// Read a FITS file from disk
    ///
    /// # Arguments
    ///
    /// * `file` - The name of the file to read
    ///
    /// # Returns
    ///
    /// A FITS structure containing the contents of the file
    ///
    pub fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut fits = FITS::default();

        // Read the file and parse the header
        // Create a stream for the file
        let mut file = std::fs::File::open(file)?;
        let mut rawbytes = Vec::new();
        file.read_to_end(&mut rawbytes)?;

        // The FITS file is a concatenation of
        // Header and Data units.  Read them in sequentially
        let mut offset = 0;
        while offset < rawbytes.len() {
            let (hdu, nbytes) = HDU::from_bytes(&rawbytes[offset..])?;
            fits.hdus.push(hdu);
            offset += nbytes;
        }
        Ok(fits)
    }
}

// indexing the fits structure just indexes the HDUs
impl std::ops::Index<usize> for FITS {
    type Output = HDU;

    fn index(&self, index: usize) -> &Self::Output {
        &self.hdus[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fits_from_file1() {
        let fits = FITS::from_file("testfiles/WFPC2u5780205r_c0fx.fits");
        match fits {
            Ok(_fits) => {}
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }

    #[test]
    fn test_fits_from_file2() {
        let fits = FITS::from_file("testfiles/FGSf64y0106m_a1f.fits");
        match fits {
            Ok(fits) => {
                println!("{}", fits[1]);
            }
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }

    #[test]
    fn test_bintable() {
        let fits = FITS::from_file("testfiles/IUElwp25637mxlo.fits");
        match fits {
            Ok(fits) => {
                if let crate::HDUData::BinTable(bt) = &fits[1].data {
                    println!("rows = {}", bt.nrows);
                    println!("cols = {}", bt.ncols);
                    for col in 0..bt.ncols {
                        println!("col {} = {:?}", col, bt.fieldname[col]);
                    }
                    for row in 0..bt.nrows {                        
                        for col in 0..bt.ncols {
                            println!("{},{} = {:?}", row, col, bt.at(row, col).unwrap());
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }

    #[test]
    fn test_fits_from_file3() {
        let fits = FITS::from_file("testfiles/FOCx38i0101t_c0f.fits");
        match fits {
            Ok(fits) => {
                if let crate::HDUData::Image(im) = &fits[0].data {
                    println!("wcs = {:?}", im.wcs);
                    println!("fits = {}", fits[1]);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }

    #[test]
    fn test_fits_from_file() {
        let fits = FITS::from_file("testfiles/EUVEngc4151imgx.fits");
        match fits {
            Ok(_fits) => {}
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }
}
