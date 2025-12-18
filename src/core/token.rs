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
}
