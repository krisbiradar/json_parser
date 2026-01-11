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
    tokens: Vec<Token>,
    token_idx: usize,
    current_byte: Option<u8>,
    eof_returned: bool,
}

impl Tokenizer {
    pub fn new(json_string: Option<String>, file_path: Option<String>) -> Self {
        match (json_string, file_path) {
            (Some(_str), Some(_path)) => {
                panic!("both json_string and file_path are supplied! choose one...");
            }
            (Some(str), None) => Self {
                reader: Box::new(BufferedStringReader::new(str.as_bytes().to_vec())),
                tokens: Vec::new(),
                token_idx: 0,
                current_byte: None,
                eof_returned: false,
            },
            (None, Some(path_string)) => {
                let path = Path::new(&path_string);
                if !path.exists() {
                    panic!("the file_path : {} doesn't exist", path_string);
                } else {
                    Self {
                        reader: Box::new(BufferedFileReader::new(path.to_path_buf())),
                        tokens: Vec::new(),
                        token_idx: 0,
                        current_byte: None,
                        eof_returned: false,
                    }
                }
            }
            (None, None) => {
                panic!("neither json_string nor file_path was provided");
            }
        }
    }

    pub fn from_string(json_string: String) -> Self {
        Self::new(Some(json_string), None)
    }

    pub fn from_file(file_path: String) -> Self {
        Self::new(None, Some(file_path))
    }

    fn advance(&mut self) -> Option<u8> {
        match self.reader.next_byte() {
            Ok(b) => {
                self.current_byte = Some(b);
                Some(b)
            }
            Err(_) => {
                self.current_byte = None;
                None
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(b) = self.current_byte {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                if self.advance().is_none() {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.eof_returned {
            return None;
        }

        if self.current_byte.is_none() {
            if self.advance().is_none() {
                self.token_idx += 1;
                self.eof_returned = true;
                return Some(Token::new(TokenType::EOF, self.reader.offset(), self.token_idx));
            }
        }

        self.skip_whitespace();

        if self.current_byte.is_none() {
            self.token_idx += 1;
            self.eof_returned = true;
            return Some(Token::new(TokenType::EOF, self.reader.offset(), self.token_idx));
        }

        let start_pos = self.reader.offset();
        let b = self.current_byte.unwrap();

        let token = match b {
            b'{' => {
                self.advance();
                self.token_idx += 1;
                Token::new(TokenType::LeftBrace, start_pos, self.token_idx)
            }
            b'}' => {
                self.advance();
                self.token_idx += 1;
                Token::new(TokenType::RightBrace, start_pos, self.token_idx)
            }
            b'[' => {
                self.advance();
                self.token_idx += 1;
                Token::new(TokenType::LeftSquareBracket, start_pos, self.token_idx)
            }
            b']' => {
                self.advance();
                self.token_idx += 1;
                Token::new(TokenType::RightSquareBracket, start_pos, self.token_idx)
            }
            b':' => {
                self.advance();
                self.token_idx += 1;
                Token::new(TokenType::Colon, start_pos, self.token_idx)
            }
            b',' => {
                self.advance();
                self.token_idx += 1;
                Token::new(TokenType::Comma, start_pos, self.token_idx)
            }
            b'"' => self.read_string(),
            b'-' | b'0'..=b'9' => self.read_number(),
            b't' | b'f' => self.read_boolean(),
            b'n' => self.read_null(),
            _ => {
                self.advance();
                self.token_idx += 1;
                Token::new(TokenType::Invalid, start_pos, self.token_idx)
            }
        };

        Some(token)
    }

    fn read_string(&mut self) -> Token {
        let start_pos = self.reader.offset();
        self.advance();

        let mut string_value = Vec::new();
        let mut escaped = false;

        loop {
            match self.current_byte {
                None => {
                    self.token_idx += 1;
                    return Token::with_value(
                        TokenType::Invalid,
                        start_pos,
                        self.token_idx,
                        Box::new(String::from_utf8_lossy(&string_value).to_string()),
                    );
                }
                Some(b) => {
                    if escaped {
                        match b {
                            b'"' => string_value.push(b'"'),
                            b'\\' => string_value.push(b'\\'),
                            b'/' => string_value.push(b'/'),
                            b'b' => string_value.push(0x08),
                            b'f' => string_value.push(0x0C),
                            b'n' => string_value.push(b'\n'),
                            b'r' => string_value.push(b'\r'),
                            b't' => string_value.push(b'\t'),
                            b'u' => {
                                let mut hex = String::new();
                                for _ in 0..4 {
                                    self.advance();
                                    if let Some(h) = self.current_byte {
                                        hex.push(h as char);
                                    }
                                }
                                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                    if let Some(c) = char::from_u32(code) {
                                        let mut buf = [0u8; 4];
                                        let s = c.encode_utf8(&mut buf);
                                        string_value.extend_from_slice(s.as_bytes());
                                    }
                                }
                            }
                            _ => {
                                string_value.push(b'\\');
                                string_value.push(b);
                            }
                        }
                        escaped = false;
                        self.advance();
                    } else if b == b'\\' {
                        escaped = true;
                        self.advance();
                    } else if b == b'"' {
                        self.advance();
                        break;
                    } else {
                        string_value.push(b);
                        self.advance();
                    }
                }
            }
        }

        self.token_idx += 1;
        Token::with_value(
            TokenType::Text,
            start_pos,
            self.token_idx,
            Box::new(String::from_utf8_lossy(&string_value).to_string()),
        )
    }

    fn read_number(&mut self) -> Token {
        let start_pos = self.reader.offset();
        let mut number_str = Vec::new();

        if self.current_byte == Some(b'-') {
            number_str.push(b'-');
            self.advance();
        }

        while let Some(b) = self.current_byte {
            if b.is_ascii_digit() {
                number_str.push(b);
                self.advance();
            } else {
                break;
            }
        }

        if self.current_byte == Some(b'.') {
            number_str.push(b'.');
            self.advance();

            while let Some(b) = self.current_byte {
                if b.is_ascii_digit() {
                    number_str.push(b);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        if self.current_byte == Some(b'e') || self.current_byte == Some(b'E') {
            number_str.push(self.current_byte.unwrap());
            self.advance();

            if self.current_byte == Some(b'+') || self.current_byte == Some(b'-') {
                number_str.push(self.current_byte.unwrap());
                self.advance();
            }

            while let Some(b) = self.current_byte {
                if b.is_ascii_digit() {
                    number_str.push(b);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.token_idx += 1;
        let num_string = String::from_utf8_lossy(&number_str).to_string();

        if let Ok(num) = num_string.parse::<f64>() {
            Token::with_value(TokenType::Number, start_pos, self.token_idx, Box::new(num))
        } else {
            Token::with_value(TokenType::Invalid, start_pos, self.token_idx, Box::new(num_string))
        }
    }

    fn read_boolean(&mut self) -> Token {
        let start_pos = self.reader.offset();
        let mut word = Vec::new();

        while let Some(b) = self.current_byte {
            if b.is_ascii_alphabetic() {
                word.push(b);
                self.advance();
            } else {
                break;
            }
        }

        self.token_idx += 1;
        let word_str = String::from_utf8_lossy(&word).to_string();

        match word_str.as_str() {
            "true" => Token::with_value(TokenType::Boolean, start_pos, self.token_idx, Box::new(true)),
            "false" => Token::with_value(TokenType::Boolean, start_pos, self.token_idx, Box::new(false)),
            _ => Token::with_value(TokenType::Invalid, start_pos, self.token_idx, Box::new(word_str)),
        }
    }

    fn read_null(&mut self) -> Token {
        let start_pos = self.reader.offset();
        let mut word = Vec::new();

        while let Some(b) = self.current_byte {
            if b.is_ascii_alphabetic() {
                word.push(b);
                self.advance();
            } else {
                break;
            }
        }

        self.token_idx += 1;
        let word_str = String::from_utf8_lossy(&word).to_string();

        if word_str == "null" {
            Token::new(TokenType::Null, start_pos, self.token_idx)
        } else {
            Token::with_value(TokenType::Invalid, start_pos, self.token_idx, Box::new(word_str))
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while let Some(token) = self.next_token() {
            let is_eof = token.token_type() == TokenType::EOF;
            self.tokens.push(token);
            if is_eof {
                break;
            }
        }
        &self.tokens
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
