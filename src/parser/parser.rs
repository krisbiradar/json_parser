use crate::core::tokentype::TokenType;
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::ast::JsonValue;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for ParseError {}

pub struct JsonParser {
    tokenizer: Tokenizer,
    current_token_type: Option<TokenType>,
    current_string: Option<String>,
    current_number: Option<f64>,
    current_bool: Option<bool>,
    position: usize,
}

impl JsonParser {
    pub fn new(json_string: String) -> Self {
        Self {
            tokenizer: Tokenizer::from_string(json_string),
            current_token_type: None,
            current_string: None,
            current_number: None,
            current_bool: None,
            position: 0,
        }
    }

    pub fn from_file(file_path: String) -> Self {
        Self {
            tokenizer: Tokenizer::from_file(file_path),
            current_token_type: None,
            current_string: None,
            current_number: None,
            current_bool: None,
            position: 0,
        }
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        if let Some(token) = self.tokenizer.next_token() {
            self.position = token.start_pos();
            self.current_token_type = Some(token.token_type());

            match token.token_type() {
                TokenType::Text => {
                    self.current_string = token.as_string().cloned();
                }
                TokenType::Number => {
                    self.current_number = token.as_f64();
                }
                TokenType::Boolean => {
                    self.current_bool = token.as_bool();
                }
                _ => {}
            }
            Ok(())
        } else {
            self.current_token_type = Some(TokenType::EOF);
            Ok(())
        }
    }

    fn expect(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        if self.current_token_type == Some(token_type) {
            self.advance()
        } else {
            Err(ParseError {
                message: format!(
                    "Expected {:?}, found {:?}",
                    token_type, self.current_token_type
                ),
                position: self.position,
            })
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
        self.advance()?;
        let value = self.parse_value()?;

        // Optionally check that we've consumed all tokens
        if self.current_token_type != Some(TokenType::EOF) {
            return Err(ParseError {
                message: format!("Unexpected token after value: {:?}", self.current_token_type),
                position: self.position,
            });
        }

        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        match self.current_token_type {
            Some(TokenType::LeftBrace) => self.parse_object(),
            Some(TokenType::LeftSquareBracket) => self.parse_array(),
            Some(TokenType::Text) => {
                let s = self.current_string.take().ok_or_else(|| ParseError {
                    message: "Expected string value".to_string(),
                    position: self.position,
                })?;
                self.advance()?;
                Ok(JsonValue::String(s))
            }
            Some(TokenType::Number) => {
                let n = self.current_number.take().ok_or_else(|| ParseError {
                    message: "Expected number value".to_string(),
                    position: self.position,
                })?;
                self.advance()?;
                Ok(JsonValue::Number(n))
            }
            Some(TokenType::Boolean) => {
                let b = self.current_bool.take().ok_or_else(|| ParseError {
                    message: "Expected boolean value".to_string(),
                    position: self.position,
                })?;
                self.advance()?;
                Ok(JsonValue::Bool(b))
            }
            Some(TokenType::Null) => {
                self.advance()?;
                Ok(JsonValue::Null)
            }
            Some(TokenType::Invalid) => Err(ParseError {
                message: "Invalid token encountered".to_string(),
                position: self.position,
            }),
            Some(TokenType::EOF) => Err(ParseError {
                message: "Unexpected end of input".to_string(),
                position: self.position,
            }),
            _ => Err(ParseError {
                message: format!("Unexpected token: {:?}", self.current_token_type),
                position: self.position,
            }),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        self.expect(TokenType::LeftBrace)?;

        let mut obj = HashMap::new();

        // Check for empty object
        if self.current_token_type == Some(TokenType::RightBrace) {
            self.advance()?;
            return Ok(JsonValue::Object(obj));
        }

        loop {
            // Parse key (must be a string)
            if self.current_token_type != Some(TokenType::Text) {
                return Err(ParseError {
                    message: format!("Expected string key in object, found {:?}", self.current_token_type),
                    position: self.position,
                });
            }

            let key = self.current_string.take().ok_or_else(|| ParseError {
                message: "Expected string key".to_string(),
                position: self.position,
            })?;
            self.advance()?;

            // Expect colon
            self.expect(TokenType::Colon)?;

            // Parse value
            let value = self.parse_value()?;
            obj.insert(key, value);

            // Check for comma or end of object
            match self.current_token_type {
                Some(TokenType::Comma) => {
                    self.advance()?;
                    // Handle trailing comma case
                    if self.current_token_type == Some(TokenType::RightBrace) {
                        self.advance()?;
                        break;
                    }
                }
                Some(TokenType::RightBrace) => {
                    self.advance()?;
                    break;
                }
                _ => {
                    return Err(ParseError {
                        message: format!(
                            "Expected ',' or '}}' in object, found {:?}",
                            self.current_token_type
                        ),
                        position: self.position,
                    });
                }
            }
        }

        Ok(JsonValue::Object(obj))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        self.expect(TokenType::LeftSquareBracket)?;

        let mut arr = Vec::new();

        // Check for empty array
        if self.current_token_type == Some(TokenType::RightSquareBracket) {
            self.advance()?;
            return Ok(JsonValue::Array(arr));
        }

        loop {
            // Parse value
            let value = self.parse_value()?;
            arr.push(value);

            // Check for comma or end of array
            match self.current_token_type {
                Some(TokenType::Comma) => {
                    self.advance()?;
                    // Handle trailing comma case
                    if self.current_token_type == Some(TokenType::RightSquareBracket) {
                        self.advance()?;
                        break;
                    }
                }
                Some(TokenType::RightSquareBracket) => {
                    self.advance()?;
                    break;
                }
                _ => {
                    return Err(ParseError {
                        message: format!(
                            "Expected ',' or ']' in array, found {:?}",
                            self.current_token_type
                        ),
                        position: self.position,
                    });
                }
            }
        }

        Ok(JsonValue::Array(arr))
    }
}

pub fn parse(json_string: &str) -> Result<JsonValue, ParseError> {
    let mut parser = JsonParser::new(json_string.to_string());
    parser.parse()
}

pub fn parse_file(file_path: &str) -> Result<JsonValue, ParseError> {
    let mut parser = JsonParser::from_file(file_path.to_string());
    parser.parse()
}

/// Converts a JsonValue to a JSON string (compact format)
pub fn stringify(value: &JsonValue) -> String {
    value.to_string()
}

/// Converts a JsonValue to a pretty-printed JSON string with indentation
pub fn stringify_pretty(value: &JsonValue) -> String {
    stringify_with_indent(value, 0)
}

fn stringify_with_indent(value: &JsonValue, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let next_indent = "  ".repeat(indent + 1);

    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => {
            if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        JsonValue::String(s) => format!("\"{}\"", escape_json_string(s)),
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format!("{}{}", next_indent, stringify_with_indent(v, indent + 1)))
                    .collect();
                format!("[\n{}\n{}]", items.join(",\n"), indent_str)
            }
        }
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}\"{}\": {}",
                            next_indent,
                            escape_json_string(k),
                            stringify_with_indent(v, indent + 1)
                        )
                    })
                    .collect();
                format!("{{\n{}\n{}}}", items.join(",\n"), indent_str)
            }
        }
    }
}

fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}
