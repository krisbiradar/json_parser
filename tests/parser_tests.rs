use json_parser::{parse, stringify, stringify_pretty, JsonValue};

#[test]
fn parse_null() {
    let result = parse("null").unwrap();
    assert!(result.is_null());
}

#[test]
fn parse_true() {
    let result = parse("true").unwrap();
    assert_eq!(result.as_bool(), Some(true));
}

#[test]
fn parse_false() {
    let result = parse("false").unwrap();
    assert_eq!(result.as_bool(), Some(false));
}

#[test]
fn parse_integer() {
    let result = parse("42").unwrap();
    assert_eq!(result.as_f64(), Some(42.0));
    assert_eq!(result.as_i64(), Some(42));
}

#[test]
fn parse_negative_integer() {
    let result = parse("-123").unwrap();
    assert_eq!(result.as_f64(), Some(-123.0));
    assert_eq!(result.as_i64(), Some(-123));
}

#[test]
fn parse_float() {
    let result = parse("3.14159").unwrap();
    let val = result.as_f64().unwrap();
    assert!((val - 3.14159).abs() < 0.00001);
}

#[test]
fn parse_scientific_notation() {
    let result = parse("1.5e10").unwrap();
    assert_eq!(result.as_f64(), Some(1.5e10));
}

#[test]
fn parse_negative_exponent() {
    let result = parse("2.5E-4").unwrap();
    let val = result.as_f64().unwrap();
    assert!((val - 2.5e-4).abs() < 1e-10);
}

#[test]
fn parse_string() {
    let result = parse(r#""hello world""#).unwrap();
    assert_eq!(result.as_str(), Some("hello world"));
}

#[test]
fn parse_string_with_escapes() {
    let result = parse(r#""hello\nworld""#).unwrap();
    assert_eq!(result.as_str(), Some("hello\nworld"));
}

#[test]
fn parse_string_with_unicode() {
    let result = parse(r#""hello \u0041""#).unwrap();
    assert_eq!(result.as_str(), Some("hello A"));
}

#[test]
fn parse_empty_array() {
    let result = parse("[]").unwrap();
    assert!(result.is_array());
    assert_eq!(result.as_array().unwrap().len(), 0);
}

#[test]
fn parse_array_with_numbers() {
    let result = parse("[1, 2, 3]").unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0].as_f64(), Some(1.0));
    assert_eq!(arr[1].as_f64(), Some(2.0));
    assert_eq!(arr[2].as_f64(), Some(3.0));
}

#[test]
fn parse_array_with_mixed_types() {
    let result = parse(r#"[1, "hello", true, null]"#).unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 4);
    assert_eq!(arr[0].as_f64(), Some(1.0));
    assert_eq!(arr[1].as_str(), Some("hello"));
    assert_eq!(arr[2].as_bool(), Some(true));
    assert!(arr[3].is_null());
}

#[test]
fn parse_nested_array() {
    let result = parse("[[1, 2], [3, 4]]").unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0].as_array().unwrap().len(), 2);
    assert_eq!(arr[1].as_array().unwrap().len(), 2);
}

#[test]
fn parse_empty_object() {
    let result = parse("{}").unwrap();
    assert!(result.is_object());
    assert_eq!(result.as_object().unwrap().len(), 0);
}

#[test]
fn parse_simple_object() {
    let result = parse(r#"{"name": "John", "age": 30}"#).unwrap();
    assert!(result.is_object());
    assert_eq!(result.get("name").unwrap().as_str(), Some("John"));
    assert_eq!(result.get("age").unwrap().as_f64(), Some(30.0));
}

#[test]
fn parse_nested_object() {
    let result = parse(r#"{"person": {"name": "John", "age": 30}}"#).unwrap();
    let person = result.get("person").unwrap();
    assert_eq!(person.get("name").unwrap().as_str(), Some("John"));
    assert_eq!(person.get("age").unwrap().as_f64(), Some(30.0));
}

#[test]
fn parse_object_with_array() {
    let result = parse(r#"{"numbers": [1, 2, 3]}"#).unwrap();
    let numbers = result.get("numbers").unwrap().as_array().unwrap();
    assert_eq!(numbers.len(), 3);
}

#[test]
fn parse_complex_json() {
    let json = r#"{
        "name": "Test",
        "version": 1.0,
        "enabled": true,
        "Kris":"Val",
        "items": [
            {"id": 1, "value": "first" , "arr": [1,2,3,45], "otherArr":[[[]]]},
            {"id": 2, "value": "second"}
        ],
        "metadata": null
    }"#;

    let result = parse(json).unwrap();
    assert_eq!(result.get("name").unwrap().as_str(), Some("Test"));
    assert_eq!(result.get("Kris").unwrap().as_str(),Some("Val"));
    assert_eq!(result.get("version").unwrap().as_f64(), Some(1.0));
    assert_eq!(result.get("enabled").unwrap().as_bool(), Some(true));
    assert!(result.get("metadata").unwrap().is_null());

    let items = result.get("items").unwrap().as_array().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].get("id").unwrap().as_f64(), Some(1.0));
    assert_eq!(items[0].get("value").unwrap().as_str(), Some("first"));
}

#[test]
fn parse_whitespace_handling() {
    let json = r#"
    {
        "key"   :   "value"   ,
        "array" :   [   1   ,   2   ,   3   ]
    }
    "#;

    let result = parse(json).unwrap();
    assert_eq!(result.get("key").unwrap().as_str(), Some("value"));
    assert_eq!(result.get("array").unwrap().as_array().unwrap().len(), 3);
}

#[test]
fn json_value_display() {
    let obj = parse(r#"{"key": "value"}"#).unwrap();
    let display = format!("{}", obj);
    assert!(display.contains("\"key\""));
    assert!(display.contains("\"value\""));
}

#[test]
fn json_value_from_types() {
    let bool_val: JsonValue = true.into();
    assert_eq!(bool_val.as_bool(), Some(true));

    let num_val: JsonValue = 42i32.into();
    assert_eq!(num_val.as_f64(), Some(42.0));

    let str_val: JsonValue = "hello".into();
    assert_eq!(str_val.as_str(), Some("hello"));
}

#[test]
fn parse_error_on_invalid_json() {
    let result = parse("{invalid}");
    assert!(result.is_err());
}

#[test]
fn parse_error_on_unclosed_object() {
    let result = parse(r#"{"key": "value""#);
    assert!(result.is_err());
}

#[test]
fn parse_error_on_unclosed_array() {
    let result = parse("[1, 2, 3");
    assert!(result.is_err());
}

#[test]
fn stringify_simple_values() {
    assert_eq!(stringify(&JsonValue::Null), "null");
    assert_eq!(stringify(&JsonValue::Bool(true)), "true");
    assert_eq!(stringify(&JsonValue::Bool(false)), "false");
    assert_eq!(stringify(&JsonValue::Number(42.0)), "42");
    assert_eq!(stringify(&JsonValue::Number(3.14)), "3.14");
    assert_eq!(stringify(&JsonValue::String("hello".to_string())), "\"hello\"");
}

#[test]
fn stringify_array() {
    let arr = parse("[1, 2, 3]").unwrap();
    assert_eq!(stringify(&arr), "[1,2,3]");
}

#[test]
fn stringify_object() {
    let obj = parse(r#"{"a": 1}"#).unwrap();
    let s = stringify(&obj);
    assert!(s.contains("\"a\":1") || s.contains("\"a\": 1"));
}

#[test]
fn stringify_roundtrip() {
    let original = r#"{"name":"John","age":30,"active":true}"#;
    let parsed = parse(original).unwrap();
    let stringified = stringify(&parsed);
    let reparsed = parse(&stringified).unwrap();

    assert_eq!(parsed.get("name").unwrap().as_str(), reparsed.get("name").unwrap().as_str());
    assert_eq!(parsed.get("age").unwrap().as_f64(), reparsed.get("age").unwrap().as_f64());
    assert_eq!(parsed.get("active").unwrap().as_bool(), reparsed.get("active").unwrap().as_bool());
}

#[test]
fn stringify_pretty_object() {
    let obj = parse(r#"{"name": "John", "age": 30}"#).unwrap();
    let pretty = stringify_pretty(&obj);
    assert!(pretty.contains('\n'));
    assert!(pretty.contains("  ")); // indentation
}

#[test]
fn stringify_escapes_special_chars() {
    let val = JsonValue::String("hello\nworld\ttab".to_string());
    let s = stringify(&val);
    assert_eq!(s, "\"hello\\nworld\\ttab\"");
}
