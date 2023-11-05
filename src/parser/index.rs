use std::fs;
use std::io::{BufRead, Read};
use std::{error::Error, io::Cursor};

#[derive(Debug)]
pub struct Word {
    word: String,
    offset: u32,
    size: u32,
}

#[derive(Debug)]
pub struct Index<'a> {
    data: Cursor<Vec<u8>>,
    same_type_sequence: &'a str,
}

impl<'a> Index<'a> {
    pub fn new(file_path: &str, same_type_sequence: &'a str) -> Result<Self, Box<dyn Error>> {
        let buf = fs::read(file_path)?;
        Ok(Index {
            data: Cursor::new(buf),
            same_type_sequence,
        })
    }

    fn next_4bytes_as_u32(&mut self) -> u32 {
        let mut buf = [0x00u8; 4];
        self.data.read_exact(&mut buf).unwrap();
        u32::from_be_bytes(buf)
    }
}

impl<'a> Iterator for Index<'a> {
    type Item = Word;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();
        let size = self.data.read_until(0x00u8, &mut buf);
        if size.is_err() {
            return None;
        }
        let size = size.unwrap();
        if size == 0 {
            return None;
        }
        buf.pop();

        let word = String::from_utf8(buf).unwrap();
        let offset = self.next_4bytes_as_u32();
        let size = self.next_4bytes_as_u32();

        Some(Word { word, offset, size })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index() {
        let index = Index::new("testdata/stardict-oxford-gb-2.4.2/oxford-gb.idx", "m");
        assert!(index.is_ok());
        let index = index.unwrap();
        let words = index.into_iter().collect::<Vec<Word>>();
        assert_eq!(words.len(), 39429);
    }
}
