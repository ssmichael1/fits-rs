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
