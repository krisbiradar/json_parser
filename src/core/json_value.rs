use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn stringify(&self) -> String {
        let mut stringer = String::new();
        self.format_into(&mut stringer);
        stringer
    }

    fn format_into(&self, stringer: &mut String) {
        match self {
            JsonValue::Null => stringer.push_str("null"),
            JsonValue::Boolean(b) => {
                if *b {
                    stringer.push_str("true");
                } else {
                    stringer.push_str("false");
                }
            }
            JsonValue::Number(n) => stringer.push_str(&n.to_string()),
            JsonValue::String(s) => {
                stringer.push('"');
                Self::escape_string(s, stringer);
                stringer.push('"');
            }
            JsonValue::Array(arr) => {
                stringer.push('[');
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        stringer.push(',');
                    }
                    val.format_into(stringer);
                }
                stringer.push(']');
            }
            JsonValue::Object(obj) => {
                stringer.push('{');
                let mut keys: Vec<&String> = obj.keys().collect();
                keys.sort(); // sort keys for testing/determinism

                for (i, key) in keys.iter().enumerate() {
                    if i > 0 {
                        stringer.push(',');
                    }
                    stringer.push('"');
                    Self::escape_string(key, stringer);
                    stringer.push_str("\":");
                    
                    if let Some(val) = obj.get(*key) {
                        val.format_into(stringer);
                    }
                }
                stringer.push('}');
            }
        }
    }

    fn escape_string(s: &str, stringer: &mut String) {
        for c in s.chars() {
            match c {
                // standard json escapes
                '"' => stringer.push_str("\\\""),
                '\\' => stringer.push_str("\\\\"),
                '\x08' => stringer.push_str("\\b"),
                '\x0c' => stringer.push_str("\\f"),
                '\n' => stringer.push_str("\\n"),
                '\r' => stringer.push_str("\\r"),
                '\t' => stringer.push_str("\\t"),
                _ if c.is_control() => {
                    // fall back to \uXXXX loop for other control characters
                    stringer.push_str(&format!("\\u{:04x}", c as u32));
                }
                _ => stringer.push(c),
            }
        }
    }
}
