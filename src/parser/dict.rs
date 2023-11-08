use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{Cursor, Read},
};

#[derive(Debug)]
pub struct Dictionary<'a> {
    data: Cursor<Vec<u8>>,
    same_type_sequence: &'a str,
}

impl<'a> Dictionary<'a> {
    pub fn new(file_path: &str, same_type_sequence: &'a str) -> Result<Self, Box<dyn Error>> {
        let f = fs::File::open(file_path)?;
        let mut zr = flate2::read::GzDecoder::new(f);
        let mut buf = Vec::new();
        zr.read_to_end(&mut buf)?;

        Ok(Dictionary {
            data: Cursor::new(buf),
            same_type_sequence,
        })
    }

    pub fn get(
        &mut self,
        offset: u64,
        size: u64,
    ) -> Result<HashMap<char, Vec<u8>>, Box<dyn Error>> {
        self.data.set_position(offset);
        for (i, c) in self.same_type_sequence.chars().enumerate() {}
    }
}
