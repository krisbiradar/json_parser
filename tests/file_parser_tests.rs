use parse_light::core::json_value::JsonValue;
use parse_light::lexer::tokenizer::Tokenizer;
use parse_light::parser::parser::Parser;
use std::path::PathBuf;

fn parse_file(file_name: &str) -> Result<JsonValue, String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(file_name);
    let tokenizer = Tokenizer::new(None, Some(path.to_string_lossy().to_string()));
    let mut parser = Parser::new(tokenizer)?;
    parser.parse()
}

// ─── Parsing sample data files ────────────────────────────────

#[test]
fn test_parse_sample1_object_from_file() {
    let res = parse_file("sample1.json");
    assert!(res.is_ok(), "Failed to parse sample1.json: {:?}", res.err());

    let value = res.unwrap();
    if let JsonValue::Object(map) = &value {
        // sample1.json is an object with a "flights" array
        let flights = map.get("flights").expect("Missing 'flights' key");
        if let JsonValue::Array(arr) = flights {
            assert_eq!(arr.len(), 10, "Expected 10 flights");

            // Verify first flight
            if let JsonValue::Object(flight) = &arr[0] {
                assert_eq!(
                    flight.get("flight_number"),
                    Some(&JsonValue::String("DL8509".to_string()))
                );
                assert_eq!(
                    flight.get("airline_name"),
                    Some(&JsonValue::String("Delta Air Lines".to_string()))
                );
                assert_eq!(
                    flight.get("departure_city"),
                    Some(&JsonValue::String("New York".to_string()))
                );
                assert_eq!(
                    flight.get("duration_hours"),
                    Some(&JsonValue::Number(20.95))
                );
            } else {
                panic!("Expected first flight to be an object");
            }

            // Verify last flight
            if let JsonValue::Object(flight) = &arr[9] {
                assert_eq!(
                    flight.get("flight_number"),
                    Some(&JsonValue::String("SA9026".to_string()))
                );
                assert_eq!(
                    flight.get("departure_country"),
                    Some(&JsonValue::String("South Africa".to_string()))
                );
            } else {
                panic!("Expected last flight to be an object");
            }
        } else {
            panic!("Expected 'flights' to be an array");
        }
    } else {
        panic!("Expected top-level object");
    }
}

#[test]
fn test_parse_sample2_array_from_file() {
    let res = parse_file("sample2.json");
    assert!(res.is_ok(), "Failed to parse sample2.json: {:?}", res.err());

    let value = res.unwrap();
    if let JsonValue::Array(arr) = &value {
        // sample2.json is an array of 10 flight objects
        assert_eq!(arr.len(), 10, "Expected 10 flights");

        // Verify a middle entry
        if let JsonValue::Object(flight) = &arr[4] {
            assert_eq!(
                flight.get("flight_number"),
                Some(&JsonValue::String("KL5073".to_string()))
            );
            assert_eq!(
                flight.get("airline_name"),
                Some(&JsonValue::String("KLM Royal Dutch Airlines".to_string()))
            );
            assert_eq!(
                flight.get("arrival_city"),
                Some(&JsonValue::String("Mexico City".to_string()))
            );
        } else {
            panic!("Expected flight at index 4 to be an object");
        }
    } else {
        panic!("Expected top-level array");
    }
}

// ─── Simple object with all basic types ───────────────────────

#[test]
fn test_parse_simple_object_from_file() {
    let res = parse_file("simple_object.json");
    assert!(res.is_ok(), "Failed to parse simple_object.json: {:?}", res.err());

    let value = res.unwrap();
    if let JsonValue::Object(map) = value {
        assert_eq!(map.get("name"), Some(&JsonValue::String("John Doe".to_string())));
        assert_eq!(map.get("age"), Some(&JsonValue::Number(30.0)));
        assert_eq!(map.get("active"), Some(&JsonValue::Boolean(true)));
        assert_eq!(map.get("address"), Some(&JsonValue::Null));
    } else {
        panic!("Expected object");
    }
}

// ─── Deeply nested structures ───────────────────────────

#[test]
fn test_parse_nested_json_from_file() {
    let res = parse_file("nested.json");
    assert!(res.is_ok(), "Failed to parse nested.json: {:?}", res.err());

    let value = res.unwrap();
    if let JsonValue::Object(root) = &value {
        assert_eq!(
            root.get("company"),
            Some(&JsonValue::String("TechCorp".to_string()))
        );

        // Check departments array
        let departments = root.get("departments").expect("Missing 'departments'");
        if let JsonValue::Array(depts) = departments {
            assert_eq!(depts.len(), 2);

            // Check first department
            if let JsonValue::Object(eng) = &depts[0] {
                assert_eq!(
                    eng.get("name"),
                    Some(&JsonValue::String("Engineering".to_string()))
                );
                // Check nested employees array
                if let Some(JsonValue::Array(employees)) = eng.get("employees") {
                    assert_eq!(employees.len(), 2);
                    // Check nested skills array inside an employee
                    if let JsonValue::Object(bob) = &employees[0] {
                        assert_eq!(
                            bob.get("name"),
                            Some(&JsonValue::String("Bob".to_string()))
                        );
                        if let Some(JsonValue::Array(skills)) = bob.get("skills") {
                            assert_eq!(skills.len(), 2);
                            assert_eq!(skills[0], JsonValue::String("rust".to_string()));
                            assert_eq!(skills[1], JsonValue::String("python".to_string()));
                        } else {
                            panic!("Expected 'skills' to be an array");
                        }
                    }
                } else {
                    panic!("Expected 'employees' to be an array");
                }
            }
        } else {
            panic!("Expected 'departments' to be an array");
        }

        // Check metadata
        if let Some(JsonValue::Object(meta)) = root.get("metadata") {
            assert_eq!(meta.get("version"), Some(&JsonValue::Number(2.0)));
            assert_eq!(meta.get("generated"), Some(&JsonValue::Boolean(true)));
        } else {
            panic!("Expected 'metadata' to be an object");
        }
    } else {
        panic!("Expected top-level object");
    }
}

// ─── Special string values ────────────────────────────────

#[test]
fn test_parse_special_strings_from_file() {
    let res = parse_file("special_strings.json");
    assert!(res.is_ok(), "Failed to parse special_strings.json: {:?}", res.err());

    let value = res.unwrap();
    if let JsonValue::Object(map) = value {
        assert_eq!(map.get("empty"), Some(&JsonValue::String("".to_string())));
        assert_eq!(
            map.get("with_spaces"),
            Some(&JsonValue::String("hello world".to_string()))
        );
        assert_eq!(
            map.get("with_newline"),
            Some(&JsonValue::String("line1\nline2".to_string()))
        );
        assert_eq!(
            map.get("with_tab"),
            Some(&JsonValue::String("col1\tcol2".to_string()))
        );
        assert_eq!(
            map.get("with_quote"),
            Some(&JsonValue::String("she said \"hi\"".to_string()))
        );
        assert_eq!(
            map.get("with_backslash"),
            Some(&JsonValue::String("path\\to\\file".to_string()))
        );
        assert_eq!(
            map.get("unicode"),
            Some(&JsonValue::String("\u{00A9} 2026".to_string()))
        );
    } else {
        panic!("Expected object");
    }
}

// ─── Error cases ────────────────────────────────

#[test]
fn test_parse_invalid_json_file() {
    let res = parse_file("invalid.json");
    assert!(res.is_err(), "Expected parsing invalid.json to fail");
}

#[test]
#[should_panic(expected = "doesnt exist")]
fn test_parse_nonexistent_file() {
    let _ = parse_file("nonexistent.json");
}

// ─── Round-trip: parse file then stringify ────────────────

#[test]
fn test_parse_file_and_stringify() {
    let res = parse_file("simple_object.json");
    assert!(res.is_ok());
    let value = res.unwrap();

    // Stringify and re-parse to verify round-trip consistency
    let stringified = parse_light::stringify(&value);
    let tokenizer = Tokenizer::new(Some(stringified.clone()), None);
    let mut parser = Parser::new(tokenizer).expect("Failed to create parser for round-trip");
    let reparsed = parser.parse().expect("Failed to re-parse stringified output");

    // Verify the values match
    if let (JsonValue::Object(original), JsonValue::Object(reparsed_map)) = (&value, &reparsed) {
        assert_eq!(original.get("name"), reparsed_map.get("name"));
        assert_eq!(original.get("age"), reparsed_map.get("age"));
        assert_eq!(original.get("active"), reparsed_map.get("active"));
        assert_eq!(original.get("address"), reparsed_map.get("address"));
    } else {
        panic!("Round-trip produced different structure");
    }
}
