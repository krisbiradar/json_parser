use json_parser::core::tokentype::TokenType;
use json_parser::lexer::tokenizer::Tokenizer;

#[test]
fn test_tokenize_empty_object() {
    let json = "{}";
    let mut tokenizer = Tokenizer::new(Some(json.to_string()), None);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[2].token_type(), TokenType::EOF);
}

#[test]
fn test_tokenize_empty_array() {
    let json = "[]";
    let mut tokenizer = Tokenizer::new(Some(json.to_string()), None);
    let tokens = tokenizer.tokenize().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type(), TokenType::LeftSquareBracket);
    assert_eq!(tokens[1].token_type(), TokenType::RightSquareBracket);
    assert_eq!(tokens[2].token_type(), TokenType::EOF);
}

#[test]
fn test_next_token_simple_object() {
    let json = r#"{ "key" : "value" }"#;
    let mut tokenizer = Tokenizer::new(Some(json.to_string()), None);

    let t1 = tokenizer.next_token().unwrap();
    assert_eq!(t1.token_type(), TokenType::LeftBrace);

    let t2 = tokenizer.next_token().unwrap();
    assert_eq!(t2.token_type(), TokenType::DoubleQuote);
    assert_eq!(t2.get_value_as_string().unwrap(), "key");

    let t3 = tokenizer.next_token().unwrap();
    assert_eq!(t3.token_type(), TokenType::Colon);

    let t4 = tokenizer.next_token().unwrap();
    assert_eq!(t4.token_type(), TokenType::DoubleQuote);
    assert_eq!(t4.get_value_as_string().unwrap(), "value");

    let t5 = tokenizer.next_token().unwrap();
    assert_eq!(t5.token_type(), TokenType::RightBrace);

    let t6 = tokenizer.next_token().unwrap();
    assert_eq!(t6.token_type(), TokenType::EOF);

    // After EOF, it should error
    assert!(tokenizer.next_token().is_err());
}

#[test]
fn test_tokenize_all_types() {
    let json = r#"{
        "s": "hello",
        "n": -123.45,
        "b1": true,
        "b2": false,
        "N": null
    }"#;
    let mut tokenizer = Tokenizer::new(Some(json.to_string()), None);
    let tokens = tokenizer.tokenize().unwrap();

    let expected_types = vec![
        TokenType::LeftBrace,
        TokenType::DoubleQuote, // "s"
        TokenType::Colon,
        TokenType::DoubleQuote, // "hello"
        TokenType::Comma,
        TokenType::DoubleQuote, // "n"
        TokenType::Colon,
        TokenType::Number,      // -123.45
        TokenType::Comma,
        TokenType::DoubleQuote, // "b1"
        TokenType::Colon,
        TokenType::Boolean,     // true
        TokenType::Comma,
        TokenType::DoubleQuote, // "b2"
        TokenType::Colon,
        TokenType::Boolean,     // false
        TokenType::Comma,
        TokenType::DoubleQuote, // "N"
        TokenType::Colon,
        TokenType::Null,        // null
        TokenType::RightBrace,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected_types.len());
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.token_type(), expected_types[i]);
    }

    assert_eq!(tokens[7].get_value_as_string().unwrap(), "-123.45");
    assert_eq!(tokens[11].get_value_as_string().unwrap(), "true");
    assert_eq!(tokens[15].get_value_as_string().unwrap(), "false");
    assert!(tokens[19].value().is_none()); // Null token has no value
}

#[test]
fn test_invalid_token() {
    let json = r#"{ "key": value }"#; // value is not in quotes
    let mut tokenizer = Tokenizer::new(Some(json.to_string()), None);
    tokenizer.next_token().unwrap(); // {
    tokenizer.next_token().unwrap(); // "key"
    tokenizer.next_token().unwrap(); // :
    let err = tokenizer.next_token().unwrap_err();
    assert!(err.contains("Invalid token starting with: 'v'"));
}