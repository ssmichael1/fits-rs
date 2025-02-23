//! # FITS
//!
//! `fits` is a library for reading and writing FITS files.
//! NASA FITS (Flexible Image Transport System) is a file format commonly used in astronomy
//! as a standard for storing images and tables.  The format is defined in the
//! [FITS Standard](https://fits.gsfc.nasa.gov/standard40/fits_standard40aa-le.pdf).
//!
//! The library provides a structure for reading and writing FITS files, and interpreting
//! the contained binary data.
//!
//! ## FITS File Structure Organization
//!
//! The FITS file contains a sequence of Header and Data Units (HDUs).  Each HDU consists of a
//! header and data section.  The header is a series of 80 byte records that contain keyword/value
//! pairs.  The data section contains the actual data.  The data can be in a variety of formats
//! including images, tables, and binary tables.
//!
//! ### FITS Structure
//!
//! The FITS structure contains an array of HDUs.  These HDUs can be indexed in the FITS structure
//! itself.  For example, to access the first HDU in a FITS file, you can use the following syntax:
//! ```
//! use fits_rs::FITS;
//! let fits = FITS::from_file("samp/WFPC2u5780205r_c0fx.fits").unwrap();
//! let hdu = &fits[0];
//! ```
//!
//! ### HDU Structure
//!
//! The HDU structure contains a header and data section.  The header is a series of 80 byte records
//! that contain keyword/value pairs.  The data section contains the actual data.  The data can be in
//! a variety of formats including images, tables, and binary tables.
//!
//! ### Header Structure
//!
//! The header structure contains a series of keyword/value pairs.  The keywords are defined in the
//! FITS standard.  The header structure provides methods for accessing the keywords and their values.
//!
//!
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
//! ```
//! use fits_rs::FITS;
//! let fits = FITS::from_file("samp/WFPC2u5780205r_c0fx.fits");
//!     match fits {
//!         Ok(fits) => {
//!         println!("{}", fits);
//!     }
//!     Err(e) => {
//!         println!("Error: {}", e);
//!         panic!("Error reading FITS file");
//!     }
//! }
//! ```
//!

mod bintable;
mod errors;
mod fits;
mod hdu;
mod header;
mod image;
mod table;
mod tdisp;
mod types;
mod utils;
mod wcs;

pub(crate) use header::FITSBlock;

pub use bintable::BinTable;
pub use errors::FITSError;
pub use errors::HeaderError;
pub use fits::*;
pub use hdu::HDU;
pub use header::Header;
pub use header::Keyword;
pub use header::KeywordValue;
pub use image::Image;
pub use table::Table;
pub use tdisp::TDisp;
pub use types::*;
pub use wcs::WCS;

pub type Matrix = nalgebra::DMatrix<f64>;
