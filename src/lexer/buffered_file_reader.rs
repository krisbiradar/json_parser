use crate::lexer::{byte_reader::ByteReader, constants};
use memchr::memchr;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub struct BufferedFileReader {
    path: PathBuf,
    chunk_size: usize,
    offset: usize,
    reader: Option<BufReader<File>>,
    file_size: usize,
}

impl BufferedFileReader {
    pub fn new(path: PathBuf) -> Self {
        if (!Path::exists(&path)) {
            panic!(
                "The provided path {} doesnt exist",
                path.to_str().unwrap_or_default()
            );
        } else {
            let file_size = std::fs::metadata(&path)
                .expect("Failed to get file metadata")
                .len() as usize;
            Self {
                path: path,
                offset: 0,
                chunk_size: constants::DEFAULT_CHUNK_SIZE_FILE,
                reader: None,
                file_size,
            }
        }
    }

    pub fn init_buffer(&mut self) {
        self.reader = Some(BufReader::new(
            File::open(&self.path).expect("Error opening file"),
        ));
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }
}

impl ByteReader for BufferedFileReader {
    fn next_byte(&mut self) -> Result<u8, String> {
        self.throw_if_consumed().unwrap();
        let buff = self.reader.as_mut().unwrap().fill_buf().ok();
        if buff.unwrap().is_empty() {
            return Err("The stream has ended".to_string());
        }
        let b = buff.unwrap()[0];
        self.reader.as_mut().unwrap().consume(1);
        self.offset = self.offset + 1;

        return Ok(b);
    }
    fn offset(&mut self) -> usize {
        return self.offset;
    }
    fn next_chunk(&mut self) -> Result<Vec<u8>, String> {
        self.throw_if_consumed().unwrap();
        let reader = self.reader.as_mut().ok_or("reader missing")?;

        let mut out = Vec::with_capacity(self.chunk_size);

        loop {
            if (out.len() == self.chunk_size) {
                break;
            }
            let buf = reader.fill_buf().map_err(|_| "read failed")?;

            if buf.is_empty() {
                if out.is_empty() {
                    return Err("The stream has ended".to_string());
                } else {
                    break;
                }
            }
            let mut n: usize = buf.len();
            if ((self.chunk_size - out.len()) < buf.len()) {
                n = self.chunk_size - out.len();
            }

            out.extend_from_slice(&buf[..n]);
            reader.consume(n);
            self.offset += n;
        }

        Ok(out)
    }

    fn next_until(&mut self, byte: u8) -> Result<Vec<u8>, String> {
        self.throw_if_consumed().unwrap();
        let mut response_vector: Vec<u8> = Vec::new();
        loop {
            let buff = self.reader.as_mut().unwrap().fill_buf().ok();
            if buff.unwrap().is_empty() {
                return Err("The stream has ended".to_string());
            }
            let n = buff.unwrap().len().min(self.chunk_size);
            let chunk = buff.unwrap()[..n].to_vec();
            if let Some(pos) = memchr(byte, &chunk) {
                response_vector.extend_from_slice(&chunk[..pos]);
                self.reader.as_mut().unwrap().consume(pos);
                self.offset += pos;
                return Ok(response_vector);
            }
            response_vector.extend_from_slice(&chunk);
            self.reader.as_mut().unwrap().consume(n);
            self.offset += n;
        }
    }
    fn next_until_any(&mut self, bytes: &[u8]) -> Result<Vec<u8>, String> {
        if self.reader.is_none() {
            self.init_buffer();
        }
        let mut result = Vec::new();
        loop {
            let chunk = self
                .reader
                .as_mut()
                .unwrap()
                .fill_buf()
                .map_err(|e| e.to_string())?;

            if chunk.is_empty() {
                return if result.is_empty() {
                    Err("The stream has ended".to_string())
                } else {
                    Ok(result)
                };
            }

            let mut min_pos = None;
            for &byte in bytes {
                // get the first occurrence of any byte in bytes and return
                // if theres some priority in the bytes then the parameters should be sent that way itself
                if let Some(pos) = memchr(byte, chunk) {
                    min_pos = Some(match min_pos {
                        None => pos,
                        Some(p) => pos.min(p),
                    });
                    break;
                }
            }

            if let Some(pos) = min_pos {
                result.extend_from_slice(&chunk[..pos]);
                self.reader.as_mut().unwrap().consume(pos);
                self.offset += pos;
                return Ok(result);
            }

            let n = chunk.len();
            result.extend_from_slice(chunk);
            self.reader.as_mut().unwrap().consume(n);
            self.offset += n;
        }
    }
    fn skip_white_space(&mut self) {
        if self.reader.is_none() {
            self.init_buffer();
        }
        let reader = self.reader.as_mut().unwrap();
        loop {
            let buf = match reader.fill_buf() {
                Ok(b) if !b.is_empty() => b,
                _ => return,
            };
            let b = buf[0];
            if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' {
                reader.consume(1);
                self.offset += 1;
            } else {
                return;
            }
        }
    }
    fn throw_if_consumed(&mut self) -> Result<(), String> {
        if self.reader.is_none() {
            self.init_buffer();
        }

        let reader = self.reader.as_mut().unwrap();
        match reader.fill_buf() {
            Ok(buf) if buf.is_empty() => Err("Input file is consumed".to_string()),
            Ok(_) => Ok(()),
            Err(_) => Err("Error reading file".to_string()),
        }
    }
}
