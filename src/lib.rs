#![allow(unused_parens)]
pub mod core;
pub mod lexer;
pub mod parser;

use std::path::Path;

use crate::core::json_value::JsonValue;
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser::Parser;

/// Parses a valid JSON string into a `JsonValue`.
pub fn parse(json: &str, file_path: &str) -> Result<JsonValue, String> {
    let tokenizer: Tokenizer;
    if Path::exists(Path::new(json)) {
        tokenizer = Tokenizer::new(None, Some(file_path.to_string()));
    } else {
        tokenizer = Tokenizer::new(Some(json.to_string()), None)
    }

    let mut parser = Parser::new(tokenizer)?;
    parser.parse()
}

/// Converts a `JsonValue` back into a valid JSON string.
pub fn stringify(value: &JsonValue) -> String {
    value.stringify()
}
