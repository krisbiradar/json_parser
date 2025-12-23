use crate::lexer::{byte_reader::ByteReader, constants};
use memchr::memchr;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek},
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
                chunk_size: constants::default_chunk_size_file,
                reader: None,
            }
        }
    }

    pub fn init_buffer(&mut self) {
        self.reader = Some(BufReader::new(
            File::open(&self.path).expect("Error opening file"),
        ));
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

    fn next_chunk(&mut self) -> Result<Vec<u8>, String> {
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
                return Ok(response_vector);
            }
            response_vector.extend_from_slice(&chunk);
            self.reader.as_mut().unwrap().consume(n);
            self.offset += n;
        }
    }
}
