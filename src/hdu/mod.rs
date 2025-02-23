use crate::types::HDUData;
use crate::BinTable;
use crate::FITSBlock;
use crate::Header;
use crate::HeaderError;
use crate::Image;
use crate::KeywordValue;
use crate::Table;
// Header and Data Unit
//
// This is comprosed of a header and optionally data (image or table)
//
// The header is a list of keywords
// The data is either an image or a table
//
#[derive(Clone, Debug)]
pub struct HDU {
    pub header: Header,
    pub data: HDUData,
}

impl Default for HDU {
    fn default() -> Self {
        HDU {
            header: Header::default(),
            data: HDUData::None,
        }
    }
}

impl HDU {
    // Get the value associated with the input keyword
    pub fn value(&self, key: &str) -> Option<&KeywordValue> {
        self.header.iter().find(|x| x.name == key).map(|x| &x.value)
    }

    /// Read a HDU from a byte array
    ///
    /// # Arguments
    ///
    /// * `rawbytes` - The byte array containing the HDU
    ///
    /// # Returns
    ///
    /// A tuple containing the HDU and the number of bytes read, or an error
    ///
    pub(crate) fn from_bytes(rawbytes: &[u8]) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        let mut record = HDU::default();
        let mut nheaders = 0;

        let mut end_found: bool = false;
        loop {
            let header = FITSBlock::from_bytes(&rawbytes[nheaders * 2880..(nheaders + 1) * 2880])?;
            for keyword in &header.0 {
                if !keyword.name.is_empty() {
                    record.header.push(keyword.clone());
                }
                if keyword.name == "END" {
                    end_found = true;
                    break;
                }
            }
            nheaders += 1;
            if end_found {
                break;
            }
        }
        let mut offset = nheaders * 2880;

        // Use the keywords to determine the data type
        if record.header.is_empty() {
            return Ok((record, offset));
        }

        match record.header[0].name.as_str() {
            "SIMPLE" => {
                // This is a primary header
                // read in an image
                let (image, nbytes) =
                    crate::Image::from_bytes(&record.header, &rawbytes[offset..])?;
                record.data = image;
                offset += nbytes;
            }
            "XTENSION" => {
                match &record.header[0].value {
                    KeywordValue::String(value) => {
                        match value.as_str() {
                            "IMAGE" => {
                                // This is an image extension
                                // read in an image
                                let (image, nbytes) =
                                    Image::from_bytes(&record.header, &rawbytes[offset..])?;
                                record.data = image;
                                offset += nbytes;
                            }
                            "TABLE" => {
                                // This is a table extension
                                // read in the table
                                let (table, nbytes) =
                                    Table::from_bytes(&record.header, &rawbytes[offset..])?;
                                record.data = table;
                                offset += nbytes;
                            }
                            "BINTABLE" => {
                                // This is a binary table extension
                                // read in the table
                                let (bintable, nbytes) =
                                    BinTable::from_bytes(&record.header, &rawbytes[offset..])?;
                                record.data = bintable;
                                offset += nbytes;
                            }

                            _ => {
                                // Unsupported extension ; report error
                                return Err(Box::new(HeaderError::UnsupportedExtension(
                                    value.clone(),
                                )));
                            }
                        }
                    }
                    _ => {
                        // Unsupported extension ; report error
                        return Err(Box::new(HeaderError::UnsupportedExtension(
                            "Extension Value not a string".to_string(),
                        )));
                    }
                }
            }
            _ => {
                // This is a header
            }
        } // end of parsing data 1st keyword type
        if offset % 2880 != 0 {
            offset += 2880 - offset % 2880;
        }
        Ok((record, offset))
    }
}

impl std::fmt::Display for HDU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for keyword in self.header.iter() {
            writeln!(f, "  {}", keyword)?;
        }
        if let HDUData::Table(t) = &self.data {
            writeln!(f, "  Table: {:?}", t)?;
        }

        Ok(())
    }
}
