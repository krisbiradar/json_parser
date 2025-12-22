use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek},
    path::{Path, PathBuf},
};

use crate::lexer::{byte_reader::ByteReader, constants};

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
                chunk_size: constants::default_buffer_size_file,
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
        return Ok(b);
    }
    
    fn next_chunk(&mut self) -> Result<Vec<u8>, String> {
        let reader = self.reader.as_mut().ok_or("reader missing")?;

        let mut out = Vec::with_capacity(self.chunk_size);

        while out.len() < self.chunk_size {
            let buf = reader.fill_buf().map_err(|_| "read failed")?;

            // EOF
            if buf.is_empty() {
                if out.is_empty() {
                    return Err("The stream has ended".to_string());
                } else {
                    break; // return what we have
                }
            }

            let remaining = self.chunk_size - out.len();
            let n = buf.len().min(remaining);

            // Copy BEFORE consuming
            out.extend_from_slice(&buf[..n]);
            reader.consume(n);
        }

        Ok(out)
    }

    fn next_until(&mut self, byte: u8) -> Result<Vec<u8>, String> {
        let buff = self.reader.as_mut().unwrap().fill_buf().ok();
        if buff.unwrap().is_empty() {
            return Err("The stream has ended".to_string());
        }
        let n = buff.unwrap().len().min(self.chunk_size);

        // Copy BEFORE consuming
        let chunk = buff.unwrap()[..n].to_vec();
        self.reader.as_mut().unwrap().consume(n);
        return Ok(chunk);
    }
}
