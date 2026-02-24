use json_parser::{parse, stringify};

#[test]
fn test_stringify_primitives() {
    assert_eq!(
        stringify(&json_parser::core::json_value::JsonValue::Null),
        "null"
    );
    assert_eq!(
        stringify(&json_parser::core::json_value::JsonValue::Boolean(true)),
        "true"
    );
    assert_eq!(
        stringify(&json_parser::core::json_value::JsonValue::Boolean(false)),
        "false"
    );
    assert_eq!(
        stringify(&json_parser::core::json_value::JsonValue::Number(123.45)),
        "123.45"
    );
}

#[test]
fn test_stringify_strings() {
    assert_eq!(
        stringify(&json_parser::core::json_value::JsonValue::String(
            "hello".to_string()
        )),
        r#""hello""#
    );

    // Test escapes
    let string_with_escapes = "line1\nline2\t\"quote\" \\slash";
    let json_val =
        json_parser::core::json_value::JsonValue::String(string_with_escapes.to_string());
    assert_eq!(stringify(&json_val), r#""line1\nline2\t\"quote\" \\slash""#);
}

#[test]
fn test_stringify_arrays() {
    let json_str = r#"[1,"two",true,null]"#;
    let val = parse(json_str).unwrap();
    assert_eq!(stringify(&val), json_str);
}

#[test]
fn test_stringify_objects() {
    // Keys should be sorted alphabetically
    let json_str = r#"{"a":1,"b":true,"c":"three"}"#;
    let val = parse(json_str).unwrap();
    assert_eq!(stringify(&val), json_str);
}

#[test]
fn test_stringify_nested() {
    let json_str = r#"{"array":[1,{"nested":"value"}],"obj":{"key":null}}"#;
    let val = parse(json_str).unwrap();
    // Re-stringifying should produce identical sorted output
    assert_eq!(stringify(&val), json_str);
}

#[test]
fn test_api_roundtrip() {
    let original = r#"{"complex":[1,2,{"test":true}],"simple":"string"}"#;
    let parsed = parse(original).expect("Should parse");
    let serialized = stringify(&parsed);
    assert_eq!(serialized, original);
}
