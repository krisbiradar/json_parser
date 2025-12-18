use std::fs;
use create::token::Token;
pub struct Lexer {
    current_pos:usize;
    current_token:Token;
    json_text:String;
    json_text_bytes:Vec<u8>;
    json_file_path:String;
}

impl Lexer {
    fn new(json_text:String) -> Self {
        return Self {
            json_text_bytes: json_text.as_bytes().to_vec(),
            json_text,
            current_pos:0
        }
    }

    fn from_file(json_file_path:String){

        return Self 
            json_file_path,
            json_text:fs::read_to_string(),
            current_pos:0,
            json_text_bytes : json_text.as_bytes().to_vec()
        }
    }

    fn next_token(& mut Self){
        if !matches!(self.current_token , None){
            move_forward(& mut self);
            current_token = Token::new()
        }
    }

    fn move_forward(& mut Self){
        Self.current_pos+=1;
    }

}