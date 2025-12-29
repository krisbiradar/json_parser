use crate::{
    core::{token::Token, tokentype::TokenType},
    lexer::{
        buffered_file_reader::BufferedFileReader, buffered_string_reader::BufferedStringReader,
        byte_reader::ByteReader,
    },
};

use std::path::Path;
pub struct Tokenizer {
    reader: Box<dyn ByteReader>,
    last_token: Option<Token>,
    unused_bytes: Vec<u8>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(json_string: Option<String>, file_path: Option<String>) -> Self {
        match (json_string, file_path) {
            (Some(_str), Some(_path)) => {
                panic!("both json_string and file_path are supplied! choose one...");
            }
            (Some(str), None) => Self {
                reader: Box::new(BufferedStringReader::new(str.as_bytes().to_vec())),
                last_token: None,
                tokens: Vec::new(),
                unused_bytes: Vec::new(),
            },
            (None, Some(path_string)) => {
                let path = Path::new(&path_string);
                if (!path.exists()) {
                    panic!("the file_path : {} doesnt exist", path_string);
                } else {
                    Self {
                        reader: Box::new(BufferedFileReader::new(path.to_path_buf())),
                        last_token: None,
                        tokens: Vec::new(),
                        unused_bytes: Vec::new(),
                    }
                }
            }
            (None, None) => {
                panic!("neither json_string nor file_path was provided");
            }
        }
    }
    pub fn next_token(&mut self) {
        //TODO Handle next_byte failing on file end or string end;
        let next_byte = self.reader.next_byte().expect("Error fetching next byte");
        self.unused_bytes.push(next_byte);
        let token_type = TokenType::get_token_type_from_byte(next_byte);
        if (token_type != TokenType::Unknown) {}
    }
    pub fn validate_token_seq(&mut self, token_type: TokenType) -> bool {
        if (self.last_token.is_none()
            && (token_type == TokenType::LeftBrace || token_type == TokenType::LeftSquareBracket))
        {
            self.last_token = Token::new(token_type, self.reader.as_mut().offset(), 1);
            return true;
        }
        
    }
}
