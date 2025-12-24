use crate::lexer::{
    buffered_file_reader::BufferedFileReader, buffered_string_reader::BufferedStringReader,
};
use std::path::{Path};
pub struct Lexer {
    buffered_file_reader: Option<BufferedFileReader>,
    buffered_string_reader: Option<BufferedStringReader>,
}

impl Lexer {
    pub fn new(json_string: Option<String>, file_path: Option<String>) -> Self {
        match (json_string, file_path) {
            (Some(_str), Some(_path)) => {
                panic!("both json_string and file_path are supplied! choose one...");
            }
            (Some(str), None) => Self {
                buffered_string_reader: Some(BufferedStringReader::new(str.as_bytes().to_vec())),
                buffered_file_reader: None,
            },
            (None, Some(path_string)) => {
                let path = Path::new(&path_string);
                if (!path.exists()) {
                    panic!("the file_path : {} doesnt exist", path_string);
                } else {
                    Self {
                        buffered_file_reader: Some(BufferedFileReader::new(path.to_path_buf())),
                        buffered_string_reader: None,
                    }
                }
            }
            (None, None) => {
                panic!("neither json_string nor file_path was provided‘");
            }
        }
    }
}
