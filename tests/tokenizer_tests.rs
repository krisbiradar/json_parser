use json_parser::lexer::tokenizer::Tokenizer;
use json_parser::core::tokentype::TokenType;

#[test]
fn tokenize_empty_object() {
    let mut tokenizer = Tokenizer::from_string("{}".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[2].token_type(), TokenType::EOF);
}

#[test]
fn tokenize_empty_array() {
    let mut tokenizer = Tokenizer::from_string("[]".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type(), TokenType::LeftSquareBracket);
    assert_eq!(tokens[1].token_type(), TokenType::RightSquareBracket);
    assert_eq!(tokens[2].token_type(), TokenType::EOF);
}

#[test]
fn tokenize_string() {
    let mut tokenizer = Tokenizer::from_string(r#""hello""#.to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), TokenType::Text);
    assert_eq!(tokens[0].as_string(), Some(&"hello".to_string()));
}

#[test]
fn tokenize_number() {
    let mut tokenizer = Tokenizer::from_string("42".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), TokenType::Number);
    assert_eq!(tokens[0].as_f64(), Some(42.0));
}

#[test]
fn tokenize_negative_number() {
    let mut tokenizer = Tokenizer::from_string("-123".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), TokenType::Number);
    assert_eq!(tokens[0].as_f64(), Some(-123.0));
}

#[test]
fn tokenize_float() {
    let mut tokenizer = Tokenizer::from_string("3.14".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), TokenType::Number);
    let val = tokens[0].as_f64().unwrap();
    assert!((val - 3.14).abs() < 0.0001);
}

#[test]
fn tokenize_boolean_true() {
    let mut tokenizer = Tokenizer::from_string("true".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), TokenType::Boolean);
    assert_eq!(tokens[0].as_bool(), Some(true));
}

#[test]
fn tokenize_boolean_false() {
    let mut tokenizer = Tokenizer::from_string("false".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), TokenType::Boolean);
    assert_eq!(tokens[0].as_bool(), Some(false));
}

#[test]
fn tokenize_null() {
    let mut tokenizer = Tokenizer::from_string("null".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), TokenType::Null);
}

#[test]
fn tokenize_simple_object() {
    let mut tokenizer = Tokenizer::from_string(r#"{"key": "value"}"#.to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type(), TokenType::Text);
    assert_eq!(tokens[1].as_string(), Some(&"key".to_string()));
    assert_eq!(tokens[2].token_type(), TokenType::Colon);
    assert_eq!(tokens[3].token_type(), TokenType::Text);
    assert_eq!(tokens[3].as_string(), Some(&"value".to_string()));
    assert_eq!(tokens[4].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[5].token_type(), TokenType::EOF);
}

#[test]
fn tokenize_array_with_values() {
    let mut tokenizer = Tokenizer::from_string("[1, 2, 3]".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0].token_type(), TokenType::LeftSquareBracket);
    assert_eq!(tokens[1].token_type(), TokenType::Number);
    assert_eq!(tokens[2].token_type(), TokenType::Comma);
    assert_eq!(tokens[3].token_type(), TokenType::Number);
    assert_eq!(tokens[4].token_type(), TokenType::Comma);
    assert_eq!(tokens[5].token_type(), TokenType::Number);
    assert_eq!(tokens[6].token_type(), TokenType::RightSquareBracket);
    assert_eq!(tokens[7].token_type(), TokenType::EOF);
}

#[test]
fn tokenize_string_with_escapes() {
    let mut tokenizer = Tokenizer::from_string(r#""hello\nworld""#.to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens[0].token_type(), TokenType::Text);
    assert_eq!(tokens[0].as_string(), Some(&"hello\nworld".to_string()));
}

#[test]
fn tokenize_string_with_escaped_quote() {
    let mut tokenizer = Tokenizer::from_string(r#""say \"hello\"""#.to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens[0].token_type(), TokenType::Text);
    assert_eq!(tokens[0].as_string(), Some(&r#"say "hello""#.to_string()));
}

#[test]
fn tokenize_scientific_notation() {
    let mut tokenizer = Tokenizer::from_string("1.5e10".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens[0].token_type(), TokenType::Number);
    assert_eq!(tokens[0].as_f64(), Some(1.5e10));
}

#[test]
fn tokenize_whitespace_handling() {
    let mut tokenizer = Tokenizer::from_string("  {  }  ".to_string());
    tokenizer.tokenize();
    let tokens = tokenizer.tokens();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[2].token_type(), TokenType::EOF);
}
