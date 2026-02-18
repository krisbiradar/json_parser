use json_parser::lexer::tokenizer::Tokenizer;
use json_parser::parser::parser::Parser;
use json_parser::core::json_value::JsonValue;

fn parse_str(json: &str) -> Result<JsonValue, String> {
    let tokenizer = Tokenizer::new(Some(json.to_string()), None);
    let mut parser = Parser::new(tokenizer).map_err(|e| e)?;
    parser.parse()
}

#[test]
fn test_empty_object() {
    let res = parse_str("{}");
    assert!(matches!(res, Ok(JsonValue::Object(map)) if map.is_empty()));
}

#[test]
fn test_empty_array() {
    let res = parse_str("[]");
    assert!(matches!(res, Ok(JsonValue::Array(vec)) if vec.is_empty()));
}

#[test]
fn test_simple_object() {
    let res = parse_str(r#"{"key": "value", "num": 123, "bool": true, "nothing": null}"#);
    match res {
        Ok(JsonValue::Object(map)) => {
            assert_eq!(map.get("key"), Some(&JsonValue::String("value".to_string())));
            assert_eq!(map.get("num"), Some(&JsonValue::Number(123.0)));
            assert_eq!(map.get("bool"), Some(&JsonValue::Boolean(true)));
            assert_eq!(map.get("nothing"), Some(&JsonValue::Null));
        }
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_nested_structures() {
    let json = r#"
    {
        "users": [
            { "id": 1, "name": "Alice" },
            { "id": 2, "name": "Bob" }
        ],
        "meta": {
            "count": 2,
            "active": true
        }
    }
    "#;
    let res = parse_str(json);
    assert!(res.is_ok());
    if let Ok(JsonValue::Object(map)) = res {
        assert!(matches!(map.get("users"), Some(JsonValue::Array(_))));
        assert!(matches!(map.get("meta"), Some(JsonValue::Object(_))));
    }
}

#[test]
fn test_invalid_missing_colon() {
    let res = parse_str(r#"{"key" "value"}"#);
    assert!(res.is_err());
}

#[test]
fn test_invalid_trailing_comma() {
    // Our parser currently expects a value after a comma in arrays
    let res = parse_str(r#"[1, 2,]"#);
    assert!(res.is_err());
}

#[test]
fn test_deeply_nested() {
    let json = r#"{"a":{"b":{"c":{"d":[]}}}}"#;
    let res = parse_str(json);
    assert!(res.is_ok());
}