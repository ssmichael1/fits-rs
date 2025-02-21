use crate::HDU;

use std::io::Read;

#[derive(Clone, Debug)]
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

impl Default for FITS {
    fn default() -> Self {
        Self::new()
    }
}

impl FITS {
    pub fn new() -> Self {
        FITS { hdus: Vec::new() }
    }

    /// indexing and return a result to ensure valid
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

    pub fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut fits = FITS::new();

        // Read the file and parse the header
        // Create a stream for the file
        let mut file = std::fs::File::open(file)?;
        let mut rawbytes = Vec::new();
        file.read_to_end(&mut rawbytes)?;

        // The FITS file is a concatenation of
        // Header and Data units.  Read them in sequentially
        let mut offset = 0;
        while offset < rawbytes.len() {
            println!("offset: {}", offset);
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
        let fits = FITS::from_file("samp/WFPC2u5780205r_c0fx.fits");
        match fits {
            Ok(fits) => {
                println!("{}", fits);
            }
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }

    #[test]
    fn test_fits_from_file2() {
        let fits = FITS::from_file("samp/FGSf64y0106m_a1f.fits");
        match fits {
            Ok(fits) => {
                println!("{}", fits[0]);
            }
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }

    #[test]
    fn test_fits_from_file3() {
        let fits = FITS::from_file("samp/FOCx38i0101t_c0f.fits");
        match fits {
            Ok(fits) => {
                if let crate::HDUData::Image(im) = &fits[0].data {
                    println!("wcs = {:?}", im.wcs);
                    //println!("fits = {}", fits[0]);
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
        let fits = FITS::from_file("samp/EUVEngc4151imgx.fits");
        match fits {
            Ok(fits) => {
                println!("{}", fits);
            }
            Err(e) => {
                println!("Error: {}", e);
                panic!("Error reading FITS file");
            }
        }
    }
}
