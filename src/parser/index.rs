use std::io::Cursor;

#[derive(Debug)]
pub struct Word {
    word: String,
    offset: u32,
    size: u32,
}

#[derive(Debug)]
pub struct Index {
    data: Cursor<Vec<u8>>,
}
