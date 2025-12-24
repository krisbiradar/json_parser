use crate::lexer::{byte_reader::ByteReader, constants};
use memchr::memchr;

pub struct BufferedStringReader {
    value: Vec<u8>,
    chunk_size: usize,
    offset: usize,
}

impl BufferedStringReader {
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            value: value,
            chunk_size: constants::DEFAULT_CHUNK_SIZE_TEXT,
            offset: 0,
        }
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }
}
impl ByteReader for BufferedStringReader {
    fn next_byte(&mut self) -> Result<u8, String> {
        if (self.offset == self.value.len() - 1) {
            return Err("The string input has ended".to_string());
        }
        self.offset = self.offset + 1;
        return Ok(self.value[self.offset]);
    }

    fn next_chunk(&mut self) -> Result<Vec<u8>, String> {
        self.offset += self.chunk_size.min((self.value.len() - 1) - self.offset);
        Ok(self.value[self.offset..(self.offset + self.chunk_size).min(self.value.len())].to_vec())
    }
    fn next_until(&mut self, byte: u8) -> Result<Vec<u8>, String> {
        if let Some(pos) = memchr(byte, &self.value[self.offset..]) {
            Ok(self.value[self.offset..pos + 1.min(self.value.len())].to_vec())
        } else {
            Err("the requested byte sequence is not found".to_string())
        }
    }
}
