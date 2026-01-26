use crate::core::tokentype::TokenType;
use std::any::Any;
pub struct Token {
    token_type: TokenType,
    value: Option<Box<dyn Any>>,
    start_pos: usize,
    end_pos: Option<usize>,
    token_idx: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start_pos: usize, token_idx: usize) -> Self {
        return Self {
            token_type,
            value: None,
            start_pos,
            end_pos: None,
            token_idx,
        };
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn with_value(
        token_type: TokenType,
        start_pos: usize,
        token_idx: usize,
        value: Box<dyn Any>,
    ) -> Self {
        return Self {
            token_type,
            value: Some(value),
            start_pos,
            end_pos: None,
            token_idx,
        };
    }

    pub fn get_value_as_string(&self) -> Option<String> {
        if let Some(ref val) = self.value {
            val.downcast_ref::<String>().map(|s| s.clone())
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&Box<dyn Any>> {
        self.value.as_ref()
    }
    pub fn to_string(&self) -> String {
        match self.token_type {
            TokenType::DoubleQuote | TokenType::Number | TokenType::Boolean | TokenType::Null | TokenType::Text => {
                if let Some(ref val) = self.value {
                    if let Some(s) = val.downcast_ref::<String>() {
                        return s.clone();
                    }
                }
                format!("{}", self.token_type)
            }
            _ => format!("{}", self.token_type),
        }
    }
    pub fn clone(&self) -> Token {
        Token::new(self.token_type, self.start_pos, self.token_idx)
    }
}
