use crate::{
    core::{token::Token, tokentype::TokenType, tokentyperelationships::TokenTypeRelationShips},
    lexer::{
        buffered_file_reader::BufferedFileReader, buffered_string_reader::BufferedStringReader,
        byte_reader::ByteReader, fsm::{FSM, FSMState},
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

    fn next_token(&mut self) -> Result<Token, String> {
        if self.fsm.current_token_idx == 0
            || self.fsm.total_bytes_consumed == self.fsm.total_bytes_to_consume - 1
        {
            return self.handle_first_last_token();
        } else {
            loop {
                if(self.fsm.processed()){
                    break;
                }

                let seq = self.reader.next_byte().expect("Error tokenizing string");

                if (!TokenType::is_single_byte_token(seq)) {
                  //lets first process boolean and null values and number tokens 
                  // we can process strings later
                  self.fsm.current_sequence.push(seq);
                  match seq  {
                    b't' | b'f' |  b'T' | b'F' => self.handle_possible_boolean(),
                    b'n' | b'N' => self.handle_possible_null(),
                    b'0'..=b'9' => self.handle_possible_number(),
                    b'"' => self.handle_string(),
                    _ => {
                        self.fsm.current_sequence.clear();
                        self.fsm.total_bytes_consumed += 1;
                        continue;
                    }

                  }
                  let chunk = self.reader.next_chunk().expect("Error fetching chunk");
                 
                } else {
                    let token = Token::new(
                        TokenType::get_token_type_from_byte(seq),
                        self.reader.offset() - 1,
                        self.fsm.current_token_idx,
                    );
                   
                    TokenTypeRelationShips::is_valid_token_sequence(
                        self.fsm.last_token().as_ref(),
                        Some(&token),
                    ).unwrap();
                    self.fsm
                        .all_tokens
                        .insert(self.fsm.current_token_idx, token.clone());
                    self.fsm.current_token_idx += 1;
                    self.fsm.total_bytes_consumed += 1;
                }
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
    fn handle_possible_boolean(& mut self){

    }
    fn handle_possible_null(& mut self){

    }
    fn handle_possible_number(& mut self){

    }
    fn handle_string(& mut self){

    }
}
