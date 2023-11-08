use std::{error::Error, fs, io::Cursor};

#[derive(Debug)]
pub struct Dictionary<'a> {
    data: Cursor<Vec<u8>>,
    same_type_sequence: &'a str,
}

impl<'a> Dictionary<'a> {
    pub fn new(file_path: &str, same_type_sequence: &'a str) -> Result<Self, Box<dyn Error>> {
        fs::File::open(file_path)?;
        let zr = flate2::read::GzDecoder::new(fs::File::open(file_path)?);
        todo!()
    }
}
