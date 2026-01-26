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
        self.throw_if_consumed().unwrap();
        let res = self.value[self.offset];
        self.offset += 1;
        Ok(res)
    }

    fn next_chunk(&mut self) -> Result<Vec<u8>, String> {
        self.throw_if_consumed().unwrap();
        let end = (self.offset + self.chunk_size).min(self.value.len());
        let res = self.value[self.offset..end].to_vec();
        self.offset = end;
        Ok(res)
    }
    fn next_until(&mut self, byte: u8) -> Result<Vec<u8>, String> {
        self.throw_if_consumed().unwrap();
        if let Some(pos) = memchr(byte, &self.value[self.offset..]) {
            let end = self.offset + pos;
            let res = self.value[self.offset..end].to_vec();
            self.offset = end ;
            Ok(res)
        } else {
            Err("the requested byte sequence is not found".to_string())
        }
    }
    
    fn next_until_any(&mut self, bytes: &[u8]) -> Result<Vec<u8>, String> {
        self.throw_if_consumed()?;
        let mut min_pos = None;

        let slice_to_search = &self.value[self.offset..];

        for &byte in bytes {
            if let Some(pos) = memchr(byte, slice_to_search) {
                min_pos = Some(match min_pos {
                    None => pos,
                    Some(p) => pos.min(p),
                });
                break;
            }
        }

        if let Some(pos) = min_pos {
            let end = self.offset + pos;
            let result = self.value[self.offset..end].to_vec();
            self.offset = end;
            Ok(result)
        } else {
            // Delimiter not found, so consume till the end
            let result = self.value[self.offset..].to_vec();
            self.offset = self.value.len();
            Ok(result)
        }
    }
    
    fn skip_white_space(&mut self) {
        while self.offset < self.value.len() {
            let b = self.value[self.offset];
            if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' {
                self.offset += 1;
            } else {
                break;
            }
        }
    }

    fn offset(& mut self) -> usize {
        return self.offset;
    }
    fn throw_if_consumed(& mut self )->Result<(),String>{
        if(self.offset >=self.value.len()){
            return Err("Input text is consumed".to_string());
        }
        Ok(())
    }
}
