use crate::{
    core::{json_value::JsonValue, token::Token, tokentype::TokenType},
    lexer::tokenizer::Tokenizer,
};
use std::collections::HashMap;

// NEW CODE — ADDITION
pub struct Parser {
    tokens: Vec<Token>,
    current_idx: usize,
}

enum Container {
    Object(HashMap<String, JsonValue>, Option<String>), // Map, Pending Key
    Array(Vec<JsonValue>),
}

#[derive(PartialEq)]
enum ParserState {
    ExpectValue,
    ExpectKey,
    ExpectColon,
    ExpectCommaOrEnd,
}

impl Parser {
    pub fn new(mut tokenizer: Tokenizer) -> Result<Self, String> {
        tokenizer.tokenize()?;
        let mut tokens: Vec<Token> = tokenizer
            .fsm
            .all_tokens
            .iter()
            .map(|(_, v)| v.clone())
            .collect();
        tokens.sort_by_key(|t| t.token_idx());
        
        Ok(Self {
            tokens,
            current_idx: 0,
        })
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        if self.tokens.is_empty() {
             return Ok(JsonValue::Null);
        }

        let mut stack: Vec<Container> = Vec::new();
        let mut state = ParserState::ExpectValue;
        let mut root_value: Option<JsonValue> = None;

        while self.current_idx < self.tokens.len() {
            let token = &self.tokens[self.current_idx];
            let token_type = token.token_type();

            match state {
                ParserState::ExpectValue => {
                    match token_type {
                        TokenType::LeftBrace => {
                            stack.push(Container::Object(HashMap::new(), None));
                            state = ParserState::ExpectKey;
                            self.advance();
                        }
                        TokenType::LeftSquareBracket => {
                            stack.push(Container::Array(Vec::new()));
                            state = ParserState::ExpectValue; // Array expects value (or end) next
                            self.advance();
                        }
                        TokenType::Text | TokenType::Number | TokenType::Boolean | TokenType::Null => {
                             let val = self.token_to_value(token)?;
                             self.advance();
                             self.insert_value(&mut stack, &mut root_value, val)?;
                             state = ParserState::ExpectCommaOrEnd;
                             if stack.is_empty() {
                                 // Done.
                             }
                        }
                        TokenType::RightSquareBracket => {
                            // Valid only if we are in an Array and it's empty "[]"
                            if let Some(Container::Array(vec)) = stack.last() {
                                if vec.is_empty() {
                                    let container = stack.pop().unwrap();
                                    let val = self.container_to_value(container);
                                    self.insert_value(&mut stack, &mut root_value, val)?;
                                    self.advance();
                                    state = ParserState::ExpectCommaOrEnd;
                                } else {
                                     return Err("Trailing comma in array".to_string());
                                }
                            } else {
                                return Err(format!("Unexpected token: {:?}", token_type));
                            }
                        }
                        // RightBrace handled in ExpectKey for empty object
                        TokenType::RightBrace => {
                             if let Some(Container::Object(_, _)) = stack.last() {
                                return Err(format!("Unexpected token: {:?}", token_type)); 
                             } else {
                                 return Err(format!("Unexpected token: {:?}", token_type));
                             }
                        }
                        _ => return Err(format!("Unexpected token: {:?}", token_type)),
                    }
                }
                ParserState::ExpectKey => {
                     match token_type {
                         TokenType::Text => {
                             let key = self.extract_string_value(token)?;
                             if let Some(Container::Object(_, pending_key)) = stack.last_mut() {
                                 *pending_key = Some(key);
                             } else {
                                 return Err("Internal Error: ExpectKey but not in Object".to_string());
                             }
                             self.advance();
                             state = ParserState::ExpectColon;
                         }
                         TokenType::RightBrace => {
                             // Empty object or End of object
                             if let Some(Container::Object(map, _)) = stack.last() {
                                 if map.is_empty() {
                                     let container = stack.pop().unwrap();
                                     let val = self.container_to_value(container);
                                     self.insert_value(&mut stack, &mut root_value, val)?;
                                     self.advance();
                                     state = ParserState::ExpectCommaOrEnd;
                                 } else {
                                     return Err("Trailing comma in object".to_string());
                                 }
                             } else {
                                 return Err(format!("Unexpected token: {:?}", token_type));
                             }
                         }
                         _ => return Err(format!("Expected Object Key or '}}', found {:?}", token_type)),
                     }
                }
                ParserState::ExpectColon => {
                    match token_type {
                        TokenType::Colon => {
                            self.advance();
                            state = ParserState::ExpectValue;
                        }
                        _ => return Err(format!("Expected ':', found {:?}", token_type)),
                    }
                }
                ParserState::ExpectCommaOrEnd => {
                    match token_type {
                        TokenType::Comma => {
                            self.advance();
                            // If in Object -> ExpectKey.
                            // If in Array -> ExpectValue.
                            // If Root -> Error (Trailing comma not allowed at root, nor multiple values).
                            if let Some(container) = stack.last() {
                                match container {
                                    Container::Object(_, _) => state = ParserState::ExpectKey,
                                    Container::Array(_) => state = ParserState::ExpectValue,
                                }
                            } else {
                                return Err("Unexpected comma at root".to_string());
                            }
                        }
                        TokenType::RightBrace => {
                             if let Some(Container::Object(_, _)) = stack.last() {
                                 let container = stack.pop().unwrap();
                                 let val = self.container_to_value(container);
                                 self.insert_value(&mut stack, &mut root_value, val)?;
                                 self.advance();
                                 // Stay in ExpectCommaOrEnd because we just finished a value (the object)
                             } else {
                                 return Err(format!("Unexpected '}}', found {:?}", token_type));
                             }
                        }
                        TokenType::RightSquareBracket => {
                             if let Some(Container::Array(_)) = stack.last() {
                                 let container = stack.pop().unwrap();
                                 let val = self.container_to_value(container);
                                 self.insert_value(&mut stack, &mut root_value, val)?;
                                 self.advance();
                                 // Stay in ExpectCommaOrEnd
                             } else {
                                 return Err(format!("Unexpected ']', found {:?}", token_type));
                             }
                        }
                        TokenType::EOF => {
                            if stack.is_empty() {
                                break;
                            } else {
                                return Err("Unexpected EOF".to_string());
                            }
                        }
                        _ => return Err(format!("Expected ',' or '}}' or ']', found {:?}", token_type)),
                    }
                }
            }
            
            // Allow one iteration to process insertion which might empty stack
            if stack.is_empty() && root_value.is_some() && state == ParserState::ExpectCommaOrEnd {
                 // Check next token is EOF
                 if self.current_idx < self.tokens.len() {
                      let next = &self.tokens[self.current_idx];
                      if next.token_type() != TokenType::EOF {
                          return Err(format!("Unexpected token after root value: {:?}", next.token_type()));
                      }
                 }
                 break;
            }
        }

        root_value.ok_or("No JSON value found".to_string())
    }

    fn insert_value(&self, stack: &mut Vec<Container>, root: &mut Option<JsonValue>, val: JsonValue) -> Result<(), String> {
        if let Some(container) = stack.last_mut() {
            match container {
                Container::Object(map, pending_key) => {
                    let key = pending_key.take().ok_or("Missing key for object value".to_string())?;
                    map.insert(key, val);
                }
                Container::Array(vec) => {
                    vec.push(val);
                }
            }
        } else {
            *root = Some(val);
        }
        Ok(())
    }

    fn container_to_value(&self, container: Container) -> JsonValue {
        match container {
            Container::Object(map, _) => JsonValue::Object(map),
            Container::Array(vec) => JsonValue::Array(vec),
        }
    }

    fn token_to_value(&self, token: &Token) -> Result<JsonValue, String> {
        // Assume correct token type (scalar)
        match token.token_type() {
             TokenType::Text => {
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
             _ => Err(format!("Not a scalar token: {:?}", token.token_type())),
        }
    }

    fn advance(&mut self) {
        if self.current_idx < self.tokens.len() {
            self.current_idx += 1;
        }
    }

    // Helper to extract string from Token's Any box
    fn extract_string_value(&self, token: &Token) -> Result<String, String> {
        let raw = token.get_value_as_string().ok_or_else(|| "Token has no value".to_string())?;
        
        // If it's just a Number/Boolean/Null, return as is (they store string rep)
        match token.token_type() {
             TokenType::Text => {
                 // Handle escapes
                 self.unescape_string(&raw)
             },
             _ => Ok(raw),
        }
    }

    fn unescape_string(&self, raw: &str) -> Result<String, String> {
        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some('/') => out.push('/'),
                    Some('b') => out.push('\x08'),
                    Some('f') => out.push('\x0c'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some('u') => {
                        // Unicode 4 hex digits
                        let mut hex = String::new();
                        for _ in 0..4 {
                            if let Some(h) = chars.next() {
                                hex.push(h);
                            } else {
                                return Err("Incomplete unicode escape".to_string());
                            }
                        }
                        let code_point = u32::from_str_radix(&hex, 16).map_err(|_| format!("Invalid unicode escape: \\u{}", hex))?;
                         if let Some(ch) = std::char::from_u32(code_point) {
                             out.push(ch);
                         } else {
                             return Err(format!("Invalid unicode scalar: \\u{}", hex));
                         }
                    }
                    Some(other) => return Err(format!("Invalid escape sequence: \\{}", other)),
                    None => return Err("Unexpected end of string in escape sequence".to_string()),
                }
            } else {
                out.push(c);
            }
        }
        Ok(out)
    }
}