use std::any::Any;
use crate::core::tokentype::TokenType;
struct Token {
    token_type: TokenType,
    value: Box<dyn Any>,
    start_pos:usize,
    end_pos:usize,
    token_idx:usize,
}
