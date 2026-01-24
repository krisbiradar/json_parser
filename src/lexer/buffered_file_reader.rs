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
}

impl BufferedFileReader {
    pub fn new(path: PathBuf) -> Self {
        if (!Path::exists(&path)) {
            panic!(
                "The provided path {} doesnt exist",
                path.to_str().unwrap_or_default()
            );
        } else {
            Self {
                path: path,
                offset: 0,
                chunk_size: constants::DEFAULT_CHUNK_SIZE_FILE,
                reader: None,
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
        if (matches!(self.reader, None)) {
            self.init_buffer();
        }
        let buff = self.reader.as_mut().unwrap().fill_buf().ok();
        if buff.unwrap().is_empty() {
            return Err("The stream has ended".to_string());
        }
        let b = buff.unwrap()[0];
        self.reader.as_mut().unwrap().consume(1);
        self.offset = self.offset + 1;

        return Ok(b);
    }
    fn offset(& mut self) -> usize {
        return self.offset;
    }
    fn next_chunk(&mut self) -> Result<Vec<u8>, String> {
        if (matches!(self.reader, None)) {
            self.init_buffer();
        }
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
        if (matches!(self.reader, None)) {
            self.init_buffer();
        }
        let mut response_vector: Vec<u8> = Vec::new();
        loop {
            let buff = self.reader.as_mut().unwrap().fill_buf().ok();
            if buff.unwrap().is_empty() {
                return Err("The stream has ended".to_string());
            }
            let n = buff.unwrap().len().min(self.chunk_size);
            let chunk = buff.unwrap()[..n].to_vec();
            if let Some(pos) = memchr(byte, &chunk) {
                response_vector.extend_from_slice(&chunk[..(pos + 1).min(chunk.len())]);
                self.reader.as_mut().unwrap().consume(pos + 1);
                self.offset += pos + 1;
                return Ok(response_vector);
            }
            response_vector.extend_from_slice(&chunk);
            self.reader.as_mut().unwrap().consume(n);
            self.offset += n;
        }
    }
    fn skip_white_space(&mut self) {
        let reader = self.reader.as_mut().unwrap();
        let mut currentIdx = self.offset;
        loop {
            let mut buf = reader.fill_buf().ok().unwrap();
            if (buf.is_empty()) {
                return;
            }
            if (buf[currentIdx] != b' ') {
                self.offset = currentIdx;
                return;
            }
            currentIdx = currentIdx+1;
            reader.consume(1);
        }
    }
}
