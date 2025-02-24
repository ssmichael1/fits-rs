use crate::errors::FITSError;
use crate::Header;
use crate::KeywordValue;
use crate::Matrix;

/// World Coordinate System transformations
/// See Chapter 8 of FITS standard, version 4
#[derive(Clone, Debug, Default)]
pub struct WCS {
    pub wcaxes: Option<usize>,
    pub ctype: Option<Vec<String>>,
    pub crval: Option<Vec<f64>>,
    pub crpix: Option<Vec<f64>>,
    pub cdelt: Option<Vec<f64>>,
    pub cunit: Option<Vec<String>>,
    pub cd: Option<Matrix>,
    pub pc: Option<Matrix>,
}

impl WCS {
    pub fn from_header(header: &Header) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let mut wcs = WCS::default();
        // See if this is explicitly set
        if let Some(kw) = header.value("WCSAXES") {
            if let KeywordValue::Int(ax) = kw {
                wcs.wcaxes = Some(*ax as usize);
            } else {
                return Err(Box::new(FITSError::UnexpectedValueType("WCSAXES".into())));
            }
        } else {
            wcs.wcaxes = None;
        }
        // Look for intermediate axes

        let mut niaxes = 0;
        while let Some(KeywordValue::String(s)) =
            header.value(format!("CUNIT{}", niaxes + 1).as_str())
        {
            if wcs.cunit.is_none() {
                wcs.cunit = Some(Vec::new())
            }
            wcs.cunit.as_mut().unwrap().push(s.clone());
            niaxes += 1;
        }
        niaxes = 0;
        while let Some(KeywordValue::String(s)) =
            header.value(format!("CTYPE{}", niaxes + 1).as_str())
        {
            if wcs.ctype.is_none() {
                wcs.ctype = Some(Vec::new());
            }
            wcs.ctype.as_mut().unwrap().push(s.clone());
            niaxes += 1;
        }
        niaxes = 0;
        while let Some(KeywordValue::Float(s)) =
            header.value(format!("CDELT{}", niaxes + 1).as_str())
        {
            if wcs.cdelt.is_none() {
                wcs.cdelt = Some(Vec::new());
            }
            wcs.cdelt.as_mut().unwrap().push(*s);
            niaxes += 1;
        }
        niaxes = 0;
        while let Some(KeywordValue::Float(s)) =
            header.value(format!("CRVAL{}", niaxes + 1).as_str())
        {
            if wcs.crval.is_none() {
                wcs.crval = Some(Vec::new());
            }
            wcs.crval.as_mut().unwrap().push(*s);
            niaxes += 1;
        }
        niaxes = 0;
        while let Some(KeywordValue::Float(s)) =
            header.value(format!("CRPIX{}", niaxes + 1).as_str())
        {
            if wcs.crpix.is_none() {
                wcs.crpix = Some(Vec::new());
            }
            wcs.crpix.as_mut().unwrap().push(*s);
            niaxes += 1;
        }

        if wcs.crpix.is_some() {
            let nj = wcs.crpix.as_ref().unwrap().len();
            let ni = wcs.crpix.as_ref().unwrap().len();

            for i in 0..ni {
                for j in 0..nj {
                    if let Some(KeywordValue::Float(s)) =
                        header.value(format!("CD{}_{}", i + 1, j + 1).as_str())
                    {
                        if wcs.cd.is_none() {
                            wcs.cd = Some(Matrix::identity(ni, nj));
                        }
                        wcs.cd.as_mut().unwrap()[(i, j)] = *s;
                    }
                }
            }
            for i in 0..ni {
                for j in 0..nj {
                    if let Some(KeywordValue::Float(s)) =
                        header.value(format!("PC{}_{}", i + 1, j + 1).as_str())
                    {
                        if wcs.pc.is_none() {
                            wcs.pc = Some(Matrix::identity(ni, nj));
                        }
                        wcs.pc.as_mut().unwrap()[(i, j)] = *s;
                    }
                }
            }
        }

        if wcs.cd.is_none()
            && wcs.pc.is_none()
            && wcs.cdelt.is_none()
            && wcs.crpix.is_none()
            && wcs.crval.is_none()
            && wcs.ctype.is_none()
            && wcs.cunit.is_none()
        {
            return Ok(None);
        }
        Ok(Some(wcs))
    }
}
