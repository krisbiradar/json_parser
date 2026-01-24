use crate::{
    core::{token::Token, tokentype::TokenType, tokentyperelationships::TokenTypeRelationShips},
    lexer::{
        buffered_file_reader::BufferedFileReader, buffered_string_reader::BufferedStringReader,
        byte_reader::ByteReader,
    },
};

use std::path::Path;
pub struct Tokenizer {
    reader: Box<dyn ByteReader>,
    last_token_type: Option<TokenType>,
    unused_bytes: Vec<u8>,
    // tokens: Vec<Token>, // Not used for streaming
}

impl Tokenizer {
    pub fn new(json_string: Option<String>, file_path: Option<String>) -> Self {
        match (json_string, file_path) {
            (Some(_str), Some(_path)) => {
                panic!("both json_string and file_path are supplied! choose one...");
            }
            (Some(str), None) => Self {
                reader: Box::new(BufferedStringReader::new(str.as_bytes().to_vec())),
                last_token_type: None,
                unused_bytes: Vec::new(),
            },
            (None, Some(path_string)) => {
                let path = Path::new(&path_string);
                if (!path.exists()) {
                    panic!("the file_path : {} doesnt exist", path_string);
                } else {
                    Self {
                        reader: Box::new(BufferedFileReader::new(path.to_path_buf())),
                        last_token_type: None,
                        unused_bytes: Vec::new(),
                    }
                }
            }
            (None, None) => {
                panic!("neither json_string nor file_path was provided");
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        let mut first_byte;
        loop {
            if !self.unused_bytes.is_empty() {
                first_byte = self.unused_bytes.remove(0);
            } else {
                match self.reader.next_byte() {
                    Ok(b) => first_byte = b,
                    Err(_) => {
                        // EOF
                        if self.last_token_type == Some(TokenType::EOF) {
                             return Err("EOF".to_string());
                        }
                        let token = Token::new(TokenType::EOF, self.reader.offset(), 0);
                        self.validate_token_seq(token.token_type())?;
                        self.last_token_type = Some(TokenType::EOF);
                        return Ok(token);
                    }
                }
            }

            if !TokenType::is_whitespace(first_byte) {
                break;
            }
        }

        let start_pos = self.reader.offset() - 1;
        let token_type = TokenType::get_token_type_from_byte(first_byte);
        let token: Token;

        match token_type {
            TokenType::LeftBrace | TokenType::RightBrace | 
            TokenType::LeftSquareBracket | TokenType::RightSquareBracket |
            TokenType::Colon | TokenType::Comma => {
                token = Token::new(token_type, start_pos, 0);
            }
            TokenType::DoubleQuote => {
                let mut content = self.reader.next_until(b'"')?;
                // Remove the closing quote from content
                if let Some(last) = content.last() {
                    if *last == b'"' {
                        content.pop();
                    }
                }
                let value = String::from_utf8_lossy(&content).to_string();
                token = Token::with_value(TokenType::DoubleQuote, start_pos, 0, Box::new(value));
            }
            TokenType::Number | TokenType::MinusSign => {
                let mut num_vec = vec![first_byte];
                loop {
                    let b = match self.reader.next_byte() {
                        Ok(b) => b,
                        Err(_) => break, 
                    };
                    if TokenType::is_single_byte_token(b) || TokenType::is_whitespace(b) {
                        self.unused_bytes.push(b);
                        break;
                    }
                    num_vec.push(b);
                }
                let value = String::from_utf8_lossy(&num_vec).to_string();
                token = Token::with_value(TokenType::Number, start_pos, 0, Box::new(value));
            }
            TokenType::Text => {
                let mut text_vec = vec![first_byte];
                loop {
                    let b = match self.reader.next_byte() {
                        Ok(b) => b,
                        Err(_) => break,
                    };
                    if TokenType::is_single_byte_token(b) || TokenType::is_whitespace(b) {
                        self.unused_bytes.push(b);
                        break;
                    }
                    text_vec.push(b);
                }
                let s = String::from_utf8_lossy(&text_vec);
                if s == "true" || s == "false" {
                    token = Token::with_value(TokenType::Boolean, start_pos, 0, Box::new(s.to_string()));
                } else if s == "null" {
                    token = Token::with_value(TokenType::Null, start_pos, 0, Box::new(s.to_string()));
                } else {
                    return Err(format!("Unknown token: {}", s));
                }
            }
            _ => return Err("Unknown token type".to_string()),
        }

        self.validate_token_seq(token.token_type())?;
        self.last_token_type = Some(token.token_type());
        Ok(token)
    }

    fn validate_token_seq(&self, token_type: TokenType) -> Result<(), String> {
        if let Some(last) = self.last_token_type {
            let allowed = TokenTypeRelationShips::get_allowed_next_tokens(last);
            if !allowed.contains(&token_type) {
                return Err(format!("Invalid token sequence: {:?} -> {:?}", last, token_type));
            }
        } else {
            // Start of file
            if token_type != TokenType::LeftBrace && token_type != TokenType::LeftSquareBracket {
                return Err("JSON must start with { or [".to_string());
            }
        }
        Ok(())
    }
}
