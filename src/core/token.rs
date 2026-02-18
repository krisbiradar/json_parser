use crate::core::tokentype::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
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

    pub fn token_idx(&self) -> usize {
        self.token_idx
    }

    pub fn with_value(
        token_type: TokenType,
        start_pos: usize,
        token_idx: usize,
        value: String,
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
        self.value.clone()
    }
    
    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }
    
    pub fn to_string(&self) -> String {
        match self.token_type {
            TokenType::DoubleQuote | TokenType::Number | TokenType::Boolean | TokenType::Null | TokenType::Text => {
                if let Some(ref val) = self.value {
                    return val.clone();
                }
                format!("{}", self.token_type)
            }
            _ => format!("{}", self.token_type),
        }
    }

}
