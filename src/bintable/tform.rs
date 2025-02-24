use std::error::Error;

use crate::HeaderError;

use std::rc::Rc;

/// The elemental types of a table column
#[derive(Clone, Debug, PartialEq, Default)]
pub enum TFormType {
    Logical,
    Bit,
    #[default]
    UnsignedByte,
    Int16,
    Int32,
    Int64,
    Char,
    Float32,
    Float64,
    Complex32,
    Complex64,
    ArrayD32(Rc<TFormType>, usize),
    ArrayD64(Rc<TFormType>, usize),
}

/// The Form (type) of a table column
/// reprsented by an elemental type and a repeat count
#[derive(Clone, Debug, Default)]
pub struct TForm {
    pub dtype: TFormType,
    pub repeats: usize,
}

fn tform_type_from_char(c: char) -> Result<TFormType, Box<dyn Error>> {
    match c {
        'L' => Ok(TFormType::Logical),
        'X' => Ok(TFormType::Bit),
        'B' => Ok(TFormType::UnsignedByte),
        'I' => Ok(TFormType::Int16),
        'J' => Ok(TFormType::Int32),
        'K' => Ok(TFormType::Int64),
        'A' => Ok(TFormType::Char),
        'E' => Ok(TFormType::Float32),
        'D' => Ok(TFormType::Float64),
        'C' => Ok(TFormType::Complex32),
        'M' => Ok(TFormType::Complex64),
        'P' => Ok(TFormType::ArrayD32(Rc::new(TFormType::default()), 0)),
        'Q' => Ok(TFormType::ArrayD64(Rc::new(TFormType::default()), 0)),
        _ => Err(Box::new(HeaderError::InvalidTForm(c.to_string()))),
    }
}

impl TForm {
    pub fn from_string(s: &str) -> Result<Self, Box<dyn Error>> {
        if s.is_empty() {
            return Err(Box::new(HeaderError::InvalidTForm(s.to_string())));
        }
        let numstr = s
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();
        let repeats = if !numstr.is_empty() {
            numstr.parse::<usize>()?
        } else {
            1
        };

        let dtype = tform_type_from_char(
            s.chars()
                .nth(numstr.len())
                .ok_or(HeaderError::InvalidTForm(s.to_string()))?,
        )?;

        if dtype == TFormType::ArrayD32(Rc::new(TFormType::default()), 0)
            || dtype == TFormType::ArrayD64(Rc::new(TFormType::default()), 0)
        {
            let tchar = s
                .chars()
                .nth(numstr.len())
                .ok_or(HeaderError::InvalidTForm(s.to_string()))?;
            let ttype = tform_type_from_char(tchar)?;

            let arrstr = s.chars().skip(numstr.len() + 1).collect::<String>();

            let mut n: usize = 0;
            if arrstr.starts_with('(') {
                let arrstr = arrstr
                    .chars()
                    .skip(1)
                    .take_while(|c| *c != ')')
                    .collect::<String>();
                n = arrstr.parse()?;
            }
            if dtype == TFormType::ArrayD32(Rc::new(TFormType::default()), 0) {
                Ok(TForm {
                    dtype: TFormType::ArrayD32(Rc::new(ttype), n),
                    repeats,
                })
            } else {
                Ok(TForm {
                    dtype: TFormType::ArrayD64(Rc::new(ttype), n),
                    repeats,
                })
            }
        } else {
            Ok(TForm { dtype, repeats })
        }
    }

    /// Return the number of bytes required to store the data type
    /// described by the TForm
    ///
    /// # Returns
    ///
    /// The number of bytes required to store the data type
    ///
    pub fn bytes(&self) -> usize {
        if self.dtype == TFormType::Bit {
            if self.repeats % 8 == 0 {
                return self.repeats / 8;
            } else {
                return self.repeats / 8 + 1;
            }
        }

        (match &self.dtype {
            TFormType::Logical => 1,
            TFormType::Bit => 1,
            TFormType::UnsignedByte => 1,
            TFormType::Int16 => 2,
            TFormType::Int32 => 4,
            TFormType::Int64 => 8,
            TFormType::Char => 1,
            TFormType::Float32 => 4,
            TFormType::Float64 => 8,
            TFormType::Complex32 => 8,
            TFormType::Complex64 => 16,
            TFormType::ArrayD32(v, r) => {
                r * TForm {
                    dtype: (**v).clone(),
                    repeats: 1,
                }
                .bytes()
            }
            TFormType::ArrayD64(v, r) => {
                r * TForm {
                    dtype: (**v).clone(),
                    repeats: 1,
                }
                .bytes()
            }
        }) * self.repeats
    }
}
