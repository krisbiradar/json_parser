use crate::{core::token::Token, lexer::tokenizer::Tokenizer};
pub struct ASTNode {
    value: Token,
    children:Vec<Token>
}


pub struct ASTree {
    tokenizer : Tokenizer,
}