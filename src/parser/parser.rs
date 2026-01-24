use crate::{
    core::{json_value::JsonValue, token::Token, tokentype::TokenType},
    lexer::tokenizer::Tokenizer,
};
use std::collections::HashMap;
use std::any::Any;

// NEW CODE — ADDITION
pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        let token = self.tokenizer.next_token()?;
        // JSON must start with an Object or Array (RFC 4627), though newer standards allow any value.
        // We will support any value for flexibility, but typically it's { or [.
        self.parse_value_from_token(token)
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        let token = self.tokenizer.next_token()?;
        self.parse_value_from_token(token)
    }

    fn parse_value_from_token(&mut self, token: Token) -> Result<JsonValue, String> {
        match token.token_type() {
            TokenType::LeftBrace => self.parse_object(),
            TokenType::LeftSquareBracket => self.parse_array(),
            TokenType::DoubleQuote => {
                let val = self.extract_string_value(token)?;
                Ok(JsonValue::String(val))
            }
            TokenType::Number => {
                let val_str = self.extract_string_value(token)?;
                let num = val_str.parse::<f64>().map_err(|_| "Invalid number format".to_string())?;
                Ok(JsonValue::Number(num))
            }
            TokenType::Boolean => {
                let val_str = self.extract_string_value(token)?;
                let bool_val = val_str.parse::<bool>().map_err(|_| "Invalid boolean format".to_string())?;
                Ok(JsonValue::Boolean(bool_val))
            }
            TokenType::Null => Ok(JsonValue::Null),
            _ => Err(format!("Unexpected token: {:?}", token.token_type())),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut map = HashMap::new();

        // Check if empty object
        // We need to peek or read. Since we can't peek easily, we read.
        // If it's '}', we are done.
        // If it's a Key, we process.
        
        loop {
            let token = self.tokenizer.next_token()?;
            
            if token.token_type() == TokenType::RightBrace {
                if !map.is_empty() {
                     // If we have items, we expect a comma before the next item, 
                     // but the loop structure below handles comma checks.
                     // If we hit '}' here, it means we either had an empty object "{}"
                     // or we just finished a value and saw '}'.
                     // However, the loop logic below consumes the comma.
                     // So if we are here, it's likely the first iteration (empty object).
                }
                break;
            }

            // Expect Key
            if token.token_type() != TokenType::DoubleQuote {
                return Err(format!("Expected Object Key (String), found {:?}", token.token_type()));
            }
            let key = self.extract_string_value(token)?;

            // Expect Colon
            let colon = self.tokenizer.next_token()?;
            if colon.token_type() != TokenType::Colon {
                return Err(format!("Expected ':', found {:?}", colon.token_type()));
            }

            // Parse Value
            let value = self.parse_value()?;
            map.insert(key, value);

            // Expect Comma or End
            let next = self.tokenizer.next_token()?;
            match next.token_type() {
                TokenType::Comma => continue, // Loop again for next key
                TokenType::RightBrace => break, // Done
                _ => return Err(format!("Expected ',' or '}}', found {:?}", next.token_type())),
            }
        }

        Ok(JsonValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        let mut vec = Vec::new();

        // Check for empty array or first element
        // Similar logic: read token. If ']', empty. Else, put back or parse.
        // Since we can't put back, we handle the first element specifically or use a loop that handles the start.
        
        // We'll read the first token to check for empty array.
        // If not empty, we parse it as a value, then loop for commas.
        
        let first_token = self.tokenizer.next_token()?;
        if first_token.token_type() == TokenType::RightSquareBracket {
            return Ok(JsonValue::Array(vec));
        }

        // Parse first element
        vec.push(self.parse_value_from_token(first_token)?);

        loop {
            let next = self.tokenizer.next_token()?;
            match next.token_type() {
                TokenType::Comma => {
                    // Expect another value
                    let val = self.parse_value()?;
                    vec.push(val);
                }
                TokenType::RightSquareBracket => break,
                _ => return Err(format!("Expected ',' or ']', found {:?}", next.token_type())),
            }
        }

        Ok(JsonValue::Array(vec))
    }

    // Helper to extract string from Token's Any box
    fn extract_string_value(&self, token: Token) -> Result<String, String> {
        // This requires accessing the private `value` field of Token. 
        // Since Token struct definition is in core/token.rs, we might need a getter there.
        // Assuming we can add a getter or access it if we change visibility.
        // For now, I will assume I can modify Token to make value public or add a helper.
        token.get_value_as_string().ok_or_else(|| "Token has no value".to_string())
    }
}