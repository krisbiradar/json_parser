use json_parser::core::tokentype::TokenType;
use json_parser::lexer::tokenizer::Tokenizer;
use json_parser::core::token::Token;

// Helper to collect tokens using the Iterator interface
fn tokenize(json: &str) -> Vec<Token> {
    let tokenizer = Tokenizer::new(Some(json.to_string()), None);
    tokenizer.collect::<Result<Vec<Token>, String>>().expect("Tokenization failed")
}

#[test]
fn test_tokenize_empty_object() {
    let tokens = tokenize("{}");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[2].token_type(), TokenType::EOF);
}

#[test]
fn test_tokenize_empty_array() {
    let tokens = tokenize("[]");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type(), TokenType::LeftSquareBracket);
    assert_eq!(tokens[1].token_type(), TokenType::RightSquareBracket);
    assert_eq!(tokens[2].token_type(), TokenType::EOF);
}

#[test]
fn test_tokenize_simple_object() {
    let tokens = tokenize(r#"{ "key" : "value" }"#);
    
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type(), TokenType::Text);
    assert_eq!(tokens[1].get_value_as_string().unwrap(), "key");
    assert_eq!(tokens[2].token_type(), TokenType::Colon);
    assert_eq!(tokens[3].token_type(), TokenType::Text);
    assert_eq!(tokens[3].get_value_as_string().unwrap(), "value");
    assert_eq!(tokens[4].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[5].token_type(), TokenType::EOF);
}

#[test]
fn test_tokenize_all_types() {
    let json = r#"{
        "n": -123.45,
        "b": true,
        "z": null
    }"#;
    let tokens = tokenize(json);

    // Validate specific values
    let number_token = tokens.iter().find(|t| t.token_type() == TokenType::Number).unwrap();
    assert_eq!(number_token.get_value_as_string().unwrap(), "123.45");
    
    let minus_sign_token = tokens.iter().find(|t| t.token_type() == TokenType::MinusSign).unwrap();
    assert_eq!(minus_sign_token.token_type(), TokenType::MinusSign);

    let bool_token = tokens.iter().find(|t| t.token_type() == TokenType::Boolean).unwrap();
    assert_eq!(bool_token.get_value_as_string().unwrap(), "true");

    let null_token = tokens.iter().find(|t| t.token_type() == TokenType::Null).unwrap();
    // Null token might have "null" as value string depending on implementation
    assert_eq!(null_token.get_value_as_string().unwrap(), "null");
}

#[test]
fn test_string_with_escapes() {
    // Tests handling of escaped quotes within a string: "say \"hello\""
    let tokens = tokenize(r#""say \"hello\"""#);
    assert_eq!(tokens.len(), 2); // String + EOF
    assert_eq!(tokens[0].token_type(), TokenType::Text);
    // The tokenizer preserves the escape sequence in the raw value
    assert_eq!(tokens[0].get_value_as_string().unwrap(), r#"say \"hello\""#);
}

#[test]
fn test_invalid_token() {
    let json = r#"{ "key": value }"#; // value is not in quotes
    let mut tokenizer = Tokenizer::new(Some(json.to_string()), None);
    
    // Consume valid tokens
    assert!(tokenizer.next().unwrap().is_ok()); // {
    assert!(tokenizer.next().unwrap().is_ok()); // "key"
    assert!(tokenizer.next().unwrap().is_ok()); // :
    
    // Expect error on 'v'
    let err = tokenizer.next();
    assert!(err.expect("Expected an error").is_err());
}

#[test]
fn test_complex_object() {
    let json = r#"{
        "name": "John Doe",
        "age": 30,
        "isStudent": false,
        "courses": ["Math", "Science"],
        "address": {
            "street": "123 Main St",
            "city": "New York"
        }
    }"#;    
    let tokens = tokenize(json);
    
    assert_eq!(tokens[0].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type(), TokenType::Text);
    assert_eq!(tokens[1].get_value_as_string().unwrap(), "name");
    assert_eq!(tokens[2].token_type(), TokenType::Colon);
    assert_eq!(tokens[3].token_type(), TokenType::Text);
    assert_eq!(tokens[3].get_value_as_string().unwrap(), "John Doe");
    assert_eq!(tokens[4].token_type(), TokenType::Comma);
    assert_eq!(tokens[5].token_type(), TokenType::Text);
    assert_eq!(tokens[5].get_value_as_string().unwrap(), "age");
    assert_eq!(tokens[6].token_type(), TokenType::Colon);
    assert_eq!(tokens[7].token_type(), TokenType::Number);
    assert_eq!(tokens[7].get_value_as_string().unwrap(), "30");
    assert_eq!(tokens[8].token_type(), TokenType::Comma);
    assert_eq!(tokens[9].token_type(), TokenType::Text);
    assert_eq!(tokens[9].get_value_as_string().unwrap(), "isStudent");
    assert_eq!(tokens[10].token_type(), TokenType::Colon);
    assert_eq!(tokens[11].token_type(), TokenType::Boolean);
    assert_eq!(tokens[11].get_value_as_string().unwrap(), "false");
    assert_eq!(tokens[12].token_type(), TokenType::Comma);
    assert_eq!(tokens[13].token_type(), TokenType::Text);
    assert_eq!(tokens[13].get_value_as_string().unwrap(), "courses");
    assert_eq!(tokens[14].token_type(), TokenType::Colon);
    assert_eq!(tokens[15].token_type(), TokenType::LeftSquareBracket);
    assert_eq!(tokens[16].token_type(), TokenType::Text);
    assert_eq!(tokens[16].get_value_as_string().unwrap(), "Math");
    assert_eq!(tokens[17].token_type(), TokenType::Comma);
    assert_eq!(tokens[18].token_type(), TokenType::Text);
    assert_eq!(tokens[18].get_value_as_string().unwrap(), "Science");
    assert_eq!(tokens[19].token_type(), TokenType::RightSquareBracket);
    assert_eq!(tokens[20].token_type(), TokenType::Comma);
    assert_eq!(tokens[21].token_type(), TokenType::Text);
    assert_eq!(tokens[21].get_value_as_string().unwrap(), "address");
    assert_eq!(tokens[22].token_type(), TokenType::Colon);
    assert_eq!(tokens[23].token_type(), TokenType::LeftBrace);
    assert_eq!(tokens[24].token_type(), TokenType::Text);
    assert_eq!(tokens[24].get_value_as_string().unwrap(), "street");
    assert_eq!(tokens[25].token_type(), TokenType::Colon);
    assert_eq!(tokens[26].token_type(), TokenType::Text);
    assert_eq!(tokens[26].get_value_as_string().unwrap(), "123 Main St");
    assert_eq!(tokens[27].token_type(), TokenType::Comma);
    assert_eq!(tokens[28].token_type(), TokenType::Text);
    assert_eq!(tokens[28].get_value_as_string().unwrap(), "city");
    assert_eq!(tokens[29].token_type(), TokenType::Colon);
    assert_eq!(tokens[30].token_type(), TokenType::Text);
    assert_eq!(tokens[30].get_value_as_string().unwrap(), "New York");
    assert_eq!(tokens[31].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[32].token_type(), TokenType::RightBrace);
    assert_eq!(tokens[33].token_type(), TokenType::EOF);
}