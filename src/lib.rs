#![allow(unused_parens)]

pub mod core;
pub mod lexer;
pub mod parser;

// Re-export main types for convenience
pub use parser::{JsonParser, JsonValue, ParseError, parse, parse_file, stringify, stringify_pretty};
