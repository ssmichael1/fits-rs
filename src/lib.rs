//! # Fits
//!
//! `fits` is a library for reading and writing FITS files.
//! NASA FITS (Flexible Image Transport System) is a file format commonly used in astronomy
//! as a standard for storing images and tables.  The format is defined in the
//! [FITS Standard](https://fits.gsfc.nasa.gov/standard40/fits_standard40aa-le.pdf).
//!
//! The library provides a structure for reading and writing FITS files, and interpreting
//! the contained binary data.
//!
//! ## Status
//!
//! The library is in the early stages of development.  The following features are currently
//! supported:
//! * Reading FITS files and parsing the headers
//! * Reading image data
//! * Reading table data
//! * Reading and parsing WCS (World Coordinate system) information
//!
//! The following features are planned:
//! * Writing FITS files
//! * Writing image data
//! * Writing table data
//! * Interpreting WCS information
//!
//!
//! # References
//!
//! - [FITS File Format](https://fits.gsfc.nasa.gov/fits_primer.html)
//! - [FITS Standard](https://fits.gsfc.nasa.gov/standard40/fits_standard40aa-le.pdf)
//! - [FITS Header Keywords](https://fits.gsfc.nasa.gov/fits_dictionary.html)
//!
//!
//! ## Example
//!
//! ```rust
//! use fits::FITS;
//! let fits = FITS::from_file("samp/WFPC2u5780205r_c0fx.fits");
//! match fits {
//!    Ok(fits) => {
//!      println!("{}", fits);
//!   }
//! Err(e) => {
//!    println!("Error: {}", e);
//!    panic!("Error reading FITS file");
//! }
//! ```
//!

mod errors;
mod fits;
mod hdu;
mod header;
mod image;
mod table;
mod types;
mod wcs;

pub use errors::HeaderError;
pub use fits::*;
pub use hdu::HDU;
pub use header::*;
pub use image::Image;
pub use table::Table;
pub use types::*;
pub use wcs::WCS;

pub type Matrix = nalgebra::DMatrix<f64>;
