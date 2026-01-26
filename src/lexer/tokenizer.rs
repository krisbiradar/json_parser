use crate::{
    core::{token::Token, tokentype::TokenType, tokentyperelationships::TokenTypeRelationShips},
    lexer::{
        buffered_file_reader::BufferedFileReader, buffered_string_reader::BufferedStringReader,
        byte_reader::ByteReader, fsm::FSM,
    },
};

use std::path::Path;
pub struct Tokenizer {
    reader: Box<dyn ByteReader>,
    fsm: FSM,
}

impl Tokenizer {
    pub fn new(json_string: Option<String>, file_path: Option<String>) -> Self {
        match (json_string, file_path) {
            (Some(_str), Some(_path)) => {
                panic!("both json_string and file_path are supplied! choose one...");
            }
            (Some(str), None) => {
                let size = str.len();
                Self {
                    reader: Box::new(BufferedStringReader::new(str.as_bytes().to_vec())),
                    fsm: FSM::new(size),
                }
            }
            (None, Some(path_string)) => {
                let path = Path::new(&path_string);
                if (!path.exists()) {
                    panic!("the file_path : {} doesnt exist", path_string);
                } else {
                    let size = std::fs::metadata(&path)
                        .expect("Failed to get file metadata")
                        .len() as usize;
                    Self {
                        reader: Box::new(BufferedFileReader::new(path.to_path_buf())),
                        fsm: FSM::new(size),
                    }
                }
            }
            (None, None) => {
                panic!("neither json_string nor file_path was provided");
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        if self.fsm.current_token_idx == 0
            || self.fsm.total_bytes_consumed == self.fsm.total_bytes_to_consume - 1
        {
            return self.handle_first_last_token();
        } else {
            if (self.fsm.processed()) {
                return Err("source already processed".to_string());
            }

            let mut seq = self.reader.next_byte().unwrap();
            if (TokenType::is_whitespace(seq)) {
                self.reader.skip_white_space();
            }
            seq = self.reader.next_byte().unwrap();
            if (!TokenType::is_single_byte_token(seq) && seq != b'"') {
                self.fsm.current_sequence.push(seq);
                return match seq {
                    b't' | b'f' | b'T' | b'F' => self.handle_boolean(seq),
                    b'n' | b'N' => self.handle_null(seq),
                    b'0'..=b'9' | b'-' => self.handle_number(seq),
                    _ => self.handle_invalid(seq),
                };
            } else if (seq == b'"') {
                self.fsm.total_bytes_consumed += 1;
                let quote_token = Token::new(
                    TokenType::DoubleQuote,
                    self.reader.offset() - 1,
                    self.fsm.current_token_idx,
                );
                self.fsm
                    .all_tokens
                    .insert(self.fsm.current_token_idx, quote_token.clone());
                let start = self.reader.offset();
                let str = self.reader.next_until(b'"').unwrap();
                let token = Token::with_value(
                    TokenType::Text,
                    start,
                    self.fsm.current_token_idx,
                    Box::new(str),
                );
                self.fsm
                    .all_tokens
                    .insert(self.fsm.current_token_idx, token.clone());
                self.fsm.current_token_idx += 1;
                return Ok(token);
            } else {
                let token = Token::new(
                    TokenType::get_token_type_from_byte(seq),
                    self.reader.offset() - 1,
                    self.fsm.current_token_idx,
                );

                TokenTypeRelationShips::is_valid_token_sequence(
                    self.fsm.last_token().as_ref(),
                    Some(&token),
                )
                .unwrap();
                self.fsm
                    .all_tokens
                    .insert(self.fsm.current_token_idx, token.clone());
                self.fsm.current_token_idx += 1;
                self.fsm.total_bytes_consumed += 1;
                return Ok(token);
            }
        }
        return Err("Something went wrong ".to_string());
    }

    fn handle_first_last_token(&mut self) -> Result<Token, String> {
        if self.fsm.current_token_idx == 0 {
            let byte = self.reader.next_byte()?;
            let token_type = TokenType::get_token_type_from_byte(byte);

            if token_type != TokenType::LeftBrace && token_type != TokenType::LeftSquareBracket {
                return Err(format!(
                    "Invalid JSON format: expected '{{' or '[', found byte {}",
                    byte
                ));
            }

            let token = Token::new(token_type, 0, self.fsm.current_token_idx);
            self.fsm
                .all_tokens
                .insert(self.fsm.current_token_idx, token.clone());
            self.fsm.current_token_idx += 1;
            self.fsm.total_bytes_consumed += 1;

            return Ok(token);
        }

        if self.fsm.total_bytes_consumed == self.fsm.total_bytes_to_consume - 1 {
            let token = Token::new(
                TokenType::EOF,
                self.fsm.total_bytes_consumed,
                self.fsm.current_token_idx,
            );
            self.fsm
                .all_tokens
                .insert(self.fsm.current_token_idx, token.clone());
            return Ok(token);
        }

        Err("Invalid state".to_string())
    }
    fn handle_boolean(&mut self, first_char: u8) -> Result<Token, String> {
        let start_pos = self.reader.offset() - 1;
        let mut bytes = self.reader.next_until_any(&[b',', b']', b'}'])?;
        bytes.insert(0, first_char);

        let s = String::from_utf8_lossy(&bytes).trim().to_lowercase();

        if s == "true" || s == "false" {
            let token = Token::with_value(
                TokenType::Boolean,
                start_pos,
                self.fsm.current_token_idx,
                Box::new(s.to_string()),
            );
            self.fsm
                .all_tokens
                .insert(self.fsm.current_token_idx, token.clone());
            self.fsm.current_token_idx += 1;
            Ok(token)
        } else {
            Err(format!("Invalid boolean: {}", s))
        }
    }

    fn handle_null(&mut self, first_char: u8) -> Result<Token, String> {
        let start_pos = self.reader.offset() - 1;
        let mut bytes = self.reader.next_until_any(&[b',', b']', b'}'])?;
        bytes.insert(0, first_char);

        let s = String::from_utf8_lossy(&bytes).trim().to_lowercase();

        if s == "null" {
            let token = Token::with_value(
                TokenType::Null,
                start_pos,
                self.fsm.current_token_idx,
                Box::new(s.to_string()),
            );
            self.fsm
                .all_tokens
                .insert(self.fsm.current_token_idx, token.clone());
            self.fsm.current_token_idx += 1;
            Ok(token)
        } else {
            Err(format!("Invalid null: {}", s))
        }
    }

    fn handle_number(&mut self, first_char: u8) -> Result<Token, String> {
        let start_pos = self.reader.offset() - 1;
        let mut bytes = self.reader.next_until_any(&[b',', b']', b'}'])?;
        bytes.insert(0, first_char);

        let s = String::from_utf8_lossy(&bytes).trim().to_string();

        let token = Token::with_value(
            TokenType::Number,
            start_pos,
            self.fsm.current_token_idx,
            Box::new(s),
        );
        self.fsm
            .all_tokens
            .insert(self.fsm.current_token_idx, token.clone());
        self.fsm.current_token_idx += 1;
        Ok(token)
    }

    fn handle_string(&mut self) -> Result<Token, String> {
        Err("Not implemented".to_string())
    }

    fn handle_invalid(&mut self, first_char: u8) -> Result<Token, String> {
        Err(format!(
            "Invalid token starting with: {}",
            first_char as char
        ))
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> 
    {
        Err("Not implemented".to_string())
    }
}
