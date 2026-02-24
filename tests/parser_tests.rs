use json_parser::core::json_value::JsonValue;
use json_parser::lexer::tokenizer::Tokenizer;
use json_parser::parser::parser::Parser;

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
            assert_eq!(
                map.get("key"),
                Some(&JsonValue::String("value".to_string()))
            );
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

#[test]
fn test_array_of_objects() {
    let json = r#"[
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]"#;
    let res = parse_str(json);
    assert!(res.is_ok());
    if let Ok(JsonValue::Array(vec)) = res {
        assert_eq!(vec.len(), 2);
        assert!(matches!(&vec[0], JsonValue::Object(_)));
        assert!(matches!(&vec[1], JsonValue::Object(_)));
    } else {
        panic!("Expected array");
    }
}

#[test]
fn test_array_of_different_types() {
    let json = r#"[1, "two", true, null, {"key": "value"}, [3, 4]]"#;
    let res = parse_str(json);
    assert!(res.is_ok());
    if let Ok(JsonValue::Array(vec)) = res {
        assert_eq!(vec.len(), 6);
        assert_eq!(vec[0], JsonValue::Number(1.0));
        assert_eq!(vec[1], JsonValue::String("two".to_string()));
        assert_eq!(vec[2], JsonValue::Boolean(true));
        assert_eq!(vec[3], JsonValue::Null);
        if let JsonValue::Object(map) = &vec[4] {
            assert_eq!(
                map.get("key"),
                Some(&JsonValue::String("value".to_string()))
            );
        } else {
            panic!("Expected object at index 4");
        }
        if let JsonValue::Array(inner) = &vec[5] {
            assert_eq!(inner.len(), 2);
            assert_eq!(inner[0], JsonValue::Number(3.0));
            assert_eq!(inner[1], JsonValue::Number(4.0));
        } else {
            panic!("Expected array at index 5");
        }
    } else {
        panic!("Expected array");
    }
}
