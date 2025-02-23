mod fitsblock;
mod keyword;

pub use fitsblock::FITSBlock;
pub use keyword::Keyword;
pub use keyword::KeywordValue;

/// A Header structure represents the header portion of a
/// FITS Header-Data Unit (HDU)
///
/// The header consists of an array of keywords
/// The array can be of arbitrary length
///
/// # Note: this is a thin wrapper around a Vec<Keyword>
///
#[derive(Clone, Debug, Default)]
pub struct Header(pub Vec<Keyword>);

impl std::ops::Deref for Header {
    type Target = Vec<Keyword>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for kw in &self.0 {
            writeln!(f, "  {}", kw)?;
        }
        Ok(())
    }
}

impl Header {
    // Iterator to the keywords
    pub fn iter(&self) -> std::slice::Iter<Keyword> {
        self.0.iter()
    }

    /// Find a keyword in the header by key name
    ///
    /// # ArgumentsIntoI
    ///
    /// * `key` - The name of the keyword to find
    ///
    /// # Returns
    ///
    /// The keyword if found, otherwise None
    ///
    pub fn find(&self, key: &str) -> Option<&Keyword> {
        self.0.iter().find(|x| x.name == key)
    }

    /// Return value given a key
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the keyword to find
    ///
    /// # Returns
    ///
    /// The value of the keyword if found, otherwise None
    ///
    pub fn value(&self, key: &str) -> Option<&KeywordValue> {
        self.0.iter().find(|x| x.name == key).map(|x| &x.value)
    }

    /// Return value if it is a string and key exists, else None
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the keyword to find
    ///
    /// # Returns
    ///
    /// The value of the keyword if found and is a string, otherwise None
    ///
    pub fn value_string(&self, key: &str) -> Option<String> {
        if let Some(KeywordValue::String(s)) = self.value(key) {
            Some(s.clone())
        } else {
            None
        }
    }

    /// Return value if it is an i64 and key exists, else None
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the keyword to find
    ///
    /// # Returns
    ///
    /// The value of the keyword if found and is an i64, otherwise None
    ///
    pub fn value_int(&self, key: &str) -> Option<i64> {
        if let Some(KeywordValue::Int(i)) = self.value(key) {
            Some(*i)
        } else {
            None
        }
    }

    /// Return value if it is a floating point and key exists, else None
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the keyword to find
    ///
    /// # Returns
    ///
    /// The value of the keyword if found and is an f64, otherwise None
    ///
    pub fn value_float(&self, key: &str) -> Option<f64> {
        if let Some(KeywordValue::Float(f)) = self.value(key) {
            Some(*f)
        } else {
            None
        }
    }

    /// Return value if it is a bool and key exists, else None
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the keyword to find
    ///
    /// # Returns
    ///
    /// The value of the keyword if found and is a bool, otherwise None
    ///
    pub fn value_bool(&self, key: &str) -> Option<bool> {
        if let Some(KeywordValue::Bool(b)) = self.value(key) {
            Some(*b)
        } else {
            None
        }
    }
}

pub struct HeaderIntoIterator<'a> {
    header: &'a Header,
    index: usize,
}

impl<'a> Iterator for HeaderIntoIterator<'a> {
    type Item = Keyword;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.header.len() {
            let kw = self.header.0[self.index].clone();
            self.index += 1;
            Some(kw)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Header {
    type Item = Keyword;
    type IntoIter = HeaderIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HeaderIntoIterator {
            header: self,
            index: 0,
        }
    }
}
