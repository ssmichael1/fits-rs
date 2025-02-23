use std::error::Error;

use crate::HeaderError;

#[derive(Clone, Debug, PartialEq)]
pub enum TFormType {
    Logical,
    Bit,
    UnsignedByte,
    Int16,
    Int32,
    Int64,
    Char,
    Float32,
    Float64,
    Complex32,
    Complex64,
    ArrayD32(i64),
    ArrayD64(i64),
}

pub struct TForm {
    pub dtype: TFormType,
    pub repeats: usize,
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
        // Get character from remaining string
        let mut dtype = match s.chars().nth(numstr.len()) {
            Some('L') => TFormType::Logical,
            Some('X') => TFormType::Bit,
            Some('B') => TFormType::UnsignedByte,
            Some('I') => TFormType::Int16,
            Some('J') => TFormType::Int32,
            Some('K') => TFormType::Int64,
            Some('A') => TFormType::Char,
            Some('E') => TFormType::Float32,
            Some('D') => TFormType::Float64,
            Some('C') => TFormType::Complex32,
            Some('M') => TFormType::Complex64,
            Some('P') => TFormType::ArrayD32(0),
            Some('Q') => TFormType::ArrayD64(0),
            _ => return Err(Box::new(HeaderError::InvalidTForm(s.to_string()))),
        };

        // D32 and D64 array must have repeats of 0 or 1
        if (dtype == TFormType::ArrayD32(0) || dtype == TFormType::ArrayD64(0)) && repeats > 1 {
            return Err(Box::new(HeaderError::InvalidTForm(s.to_string())));
        }
        let arrstr = s.chars().skip(numstr.len() + 1).collect::<String>();
        if arrstr.starts_with('(') {
            let arrstr = arrstr
                .chars()
                .skip(1)
                .take_while(|c| *c != ')')
                .collect::<String>();
            let n: i64 = arrstr.parse()?;
            if dtype == TFormType::ArrayD32(0) {
                dtype = TFormType::ArrayD32(n);
            }
            if dtype == TFormType::ArrayD64(0) {
                dtype = TFormType::ArrayD64(n);
            }
        }

        Ok(TForm { dtype, repeats })
    }

    pub fn bytes(&self) -> usize {
        if self.dtype == TFormType::Bit {
            if self.repeats % 8 == 0 {
                return self.repeats / 8;
            } else {
                return self.repeats / 8 + 1;
            }
        }

        (match self.dtype {
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
            TFormType::ArrayD32(_) => 4,
            TFormType::ArrayD64(_) => 8,
        }) * self.repeats
    }
}
