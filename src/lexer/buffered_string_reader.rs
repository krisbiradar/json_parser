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
            value,
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
        if self.offset >= self.value.len() {
            return Err("The string input has ended".to_string());
        }
        let end = (self.offset + self.chunk_size).min(self.value.len());
        let res = self.value[self.offset..end].to_vec();
        self.offset = end;
        Ok(res)
    }

    fn next_until(&mut self, byte: u8) -> Result<Vec<u8>, String> {
        if self.offset >= self.value.len() {
            return Err("The string input has ended".to_string());
        }
        if let Some(pos) = memchr(byte, &self.value[self.offset..]) {
            let absolute_pos = self.offset + pos;
            let result = self.value[self.offset..=absolute_pos].to_vec();
            self.offset = absolute_pos + 1;
            Ok(result)
        } else {
            Err("the requested byte sequence is not found".to_string())
        }
    }

    fn skip_white_space(&mut self) {
        while self.offset < self.value.len() && self.value[self.offset] == b' ' {
            self.offset += 1;
        }
    }

    fn offset(&mut self) -> usize {
        self.offset
    }
}
