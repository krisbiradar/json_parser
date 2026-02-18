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
    pub fsm: FSM,
}

// Things to remember
// we are going to ignore double quotes in strings and use strings as a single token
// this will reduce a lot of complexity going forward and make tokenizer impl easier

impl Tokenizer {
    pub fn new(json_string: Option<String>, file_path: Option<String>) -> Self {
        match (json_string, file_path) {
            (Some(_str), Some(_path)) => {
                panic!("both json_string and file_path are supplied! choose one...");
            }
            (Some(str), None) => Self {
                reader: Box::new(BufferedStringReader::new(str.as_bytes().to_vec())),
                fsm: FSM::new(),
            },
            (None, Some(path_string)) => {
                let path = Path::new(&path_string);
                if (!path.exists()) {
                    panic!("the file_path : {} doesnt exist", path_string);
                } else {
                    Self {
                        reader: Box::new(BufferedFileReader::new(path.to_path_buf())),
                        fsm: FSM::new(),
                    }
                }
            }
            (None, None) => {
                panic!("neither json_string nor file_path was provided");
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        if self.fsm.current_token_idx == 0 {
            return self.handle_first_last_token();
        } else {
            self.reader.skip_white_space();
            let seq = match self.reader.next_byte() {
                Ok(b) => b,
                Err(_) => {
                    let token = Token::new(
                        TokenType::EOF,
                        self.reader.offset(),
                        self.fsm.current_token_idx,
                    );
                    self.fsm
                        .all_tokens
                        .insert(self.fsm.current_token_idx, token.clone());
                    return Ok(token);
                }
            };

            if (!TokenType::is_single_byte_token(seq) && seq != b'"') {
                self.reader.skip_white_space();
                return match seq {
                    b't' | b'f' | b'T' | b'F' => self.handle_boolean(seq),
                    b'n' | b'N' => self.handle_null(seq),
                    b'0'..=b'9' => self.handle_number(seq),
                    _ => self.handle_invalid(seq),
                };
            } else if (seq == b'"') {
                return self.handle_string();
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
                ?;
                self.fsm
                    .all_tokens
                    .insert(self.fsm.current_token_idx, token.clone());
                self.fsm.current_token_idx += 1;
                return Ok(token);
            }
        }
    }

    fn handle_first_last_token(&mut self) -> Result<Token, String> {
        if self.fsm.current_token_idx == 0 {
            self.reader.skip_white_space();
            let byte = self.reader.next_byte()?;
            let token_type = TokenType::get_token_type_from_byte(byte);

            if token_type == TokenType::LeftBrace || token_type == TokenType::LeftSquareBracket {
                let token = Token::new(
                    token_type,
                    self.reader.offset() - 1,
                    self.fsm.current_token_idx,
                );
                self.fsm
                    .all_tokens
                    .insert(self.fsm.current_token_idx, token.clone());
                self.fsm.current_token_idx += 1;

                return Ok(token);
            }

            match byte {
                b'"' => {
                    let start_pos = self.reader.offset() - 1;
                    let mut output: Vec<u8> = Vec::new();
                    loop {
                        let b = self.reader.next_until(b'"')?;
                        output.extend_from_slice(&b);
                        self.reader.next_byte()?; // Consume quote
                        
                        let mut backslash_count = 0;
                        for &byte in output.iter().rev() {
                            if byte == b'\\' {
                                backslash_count += 1;
                            } else {
                                break;
                            }
                        }

                        if backslash_count % 2 == 1 {
                             output.push(b'"');
                        } else {
                            break;
                        }
                    }
                    let s = String::from_utf8(output).map_err(|e| e.to_string())?;
                    let token = Token::with_value(
                        TokenType::Text,
                        start_pos,
                        self.fsm.current_token_idx,
                        s,
                    );
                    self.fsm
                        .all_tokens
                        .insert(self.fsm.current_token_idx, token.clone());
                    self.fsm.current_token_idx += 1;
                    return Ok(token);
                }
                b't' | b'f' | b'T' | b'F' => return self.handle_boolean(byte),
                b'n' | b'N' => return self.handle_null(byte),
                b'0'..=b'9' => return self.handle_number(byte),
                _ => return self.handle_invalid(byte),
            }
        }

        Err("Invalid state".to_string())
    }
    fn handle_boolean(&mut self, first_char: u8) -> Result<Token, String> {
        self.reader.skip_white_space();
        let start_pos = self.reader.offset() - 1;
        let mut bytes = self.reader.next_until_any(&[b',', b']', b'}'])?;
        bytes.insert(0, first_char);

        let s = String::from_utf8_lossy(&bytes).trim().to_lowercase();

        if s == "true" || s == "false" {
            let token = Token::with_value(
                TokenType::Boolean,
                start_pos,
                self.fsm.current_token_idx,
                s.to_string(),
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
        self.reader.skip_white_space();
        let start_pos = self.reader.offset() - 1;
        let mut bytes = self.reader.next_until_any(&[b',', b']', b'}'])?;
        bytes.insert(0, first_char);

        let s = String::from_utf8_lossy(&bytes).trim().to_lowercase();

        if s == "null" {
            let token = Token::with_value(
                TokenType::Null,
                start_pos,
                self.fsm.current_token_idx,
                s.to_string(),
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
        self.reader.skip_white_space();
        let start_pos = self.reader.offset() - 1;
        let mut bytes = self.reader.next_until_any(&[b',', b']', b'}'])?;
        bytes.insert(0, first_char);

        let s = String::from_utf8_lossy(&bytes).trim().to_string();

        let token = Token::with_value(
            TokenType::Number,
            start_pos,
            self.fsm.current_token_idx,
            s,
        );
        self.fsm
            .all_tokens
            .insert(self.fsm.current_token_idx, token.clone());
        self.fsm.current_token_idx += 1;
        Ok(token)
    }

    fn handle_string(&mut self) -> Result<Token, String> {
        let start_pos = self.reader.offset() - 1;
        let mut output: Vec<u8> = Vec::new();

        loop {
            let b = self.reader.next_until(b'"')?;
            output.extend_from_slice(&b);
            
            // We are at '"'. Consume it.
            self.reader.next_byte()?;
            
            // Check if this quote is escaped by counting trailing backslashes in the output so far.
            // Escape sequence \\ is a backslash. \" is a quote.
            // So if we have odd number of backslashes at the end of `output` (before the quote we just found but that quote isn't in output yet, wait).
            // `output` contains content BEFORE the quote. 
            // If `output` ends in odd backslashes, the quote is escaped.
            // Example: `abc\"` -> output `abc\`. Ends in 1 backslash. Escaped.
            // Example: `abc\\"` -> output `abc\\`. Ends in 2 backslashes. Not escaped (backslash is escaped).
            
            let mut backslash_count = 0;
            for &byte in output.iter().rev() {
                if byte == b'\\' {
                    backslash_count += 1;
                } else {
                    break;
                }
            }

            if backslash_count % 2 == 1 {
                // Escaped. The quote is part of the string.
                // We shouldn't have consumed it? No, we consumed it from reader.
                // We need to add it to output.
                output.push(b'"');
            } else {
                // Not escaped. This is the closing quote.
                break;
            }
        }

        let s = String::from_utf8(output).map_err(|e| e.to_string())?;
        let token = Token::with_value(
            TokenType::Text,
            start_pos,
            self.fsm.current_token_idx,
            s,
        );

        TokenTypeRelationShips::is_valid_token_sequence(
            self.fsm.last_token().as_ref(),
            Some(&token),
        )?;

        self.fsm
            .all_tokens
            .insert(self.fsm.current_token_idx, token.clone());
        self.fsm.current_token_idx += 1;
        Ok(token)
    }

    fn handle_invalid(&mut self, first_char: u8) -> Result<Token, String> {
        Err(format!(
            "Invalid token starting with: {}",
            first_char as char
        ))
    }
    pub fn tokenize(&mut self) -> Result<(), String> {
        loop {
            let token = self.next_token()?;

            if token.token_type() == TokenType::EOF {
                break;
            }
        }
        return Ok(());
    }
}

impl Iterator for Tokenizer {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.fsm.all_tokens.get(&self.fsm.current_token_idx) {
            if token.token_type() == TokenType::EOF {
                return None;
            }
        }
        Some(self.next_token())
    }
}
