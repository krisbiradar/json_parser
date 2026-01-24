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
        if self.offset >= self.value.len() {
            return Err("The string input has ended".to_string());
        }
        let res = self.value[self.offset];
        self.offset += 1;
        Ok(res)
    }

    fn next_chunk(&mut self) -> Result<Vec<u8>, String> {
        let end = (self.offset + self.chunk_size).min(self.value.len());
        let res = self.value[self.offset..end].to_vec();
        self.offset = end;
        Ok(res)
    }
    fn next_until(&mut self, byte: u8) -> Result<Vec<u8>, String> {
        if let Some(pos) = memchr(byte, &self.value[self.offset..]) {
            let end = self.offset + pos + 1;
            let res = self.value[self.offset..end].to_vec();
            self.offset = end;
            Ok(res)
        } else {
            Err("the requested byte sequence is not found".to_string())
        }
    }

    fn skip_white_space(&mut self) {
        while self.offset < self.value.len() && self.value[self.offset] == b' ' {
            self.offset += 1;
        }
    }

    fn offset(& mut self) -> usize {
        return self.offset;
    }
}
