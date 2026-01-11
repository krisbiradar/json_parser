use std::collections::HashMap;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::String(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            JsonValue::Number(n) => {
                let i = *n as i64;
                if (i as f64 - *n).abs() < f64::EPSILON {
                    Some(i)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(obj) => obj.get(key),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }
}

impl fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => f.debug_list().entries(arr).finish(),
            JsonValue::Object(obj) => f.debug_map().entries(obj.iter()).finish(),
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsonValue::String(s) => write!(f, "\"{}\"", escape_string(s)),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, val)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", escape_string(key), val)?;
                }
                write!(f, "}}")
            }
        }
    }
}

fn escape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::Bool(b)
    }
}

impl From<i32> for JsonValue {
    fn from(n: i32) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<i64> for JsonValue {
    fn from(n: i64) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<f64> for JsonValue {
    fn from(n: f64) -> Self {
        JsonValue::Number(n)
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        JsonValue::String(s)
    }
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        JsonValue::String(s.to_string())
    }
}

impl<T: Into<JsonValue>> From<Vec<T>> for JsonValue {
    fn from(arr: Vec<T>) -> Self {
        JsonValue::Array(arr.into_iter().map(|v| v.into()).collect())
    }
}

impl<T: Into<JsonValue>> From<HashMap<String, T>> for JsonValue {
    fn from(obj: HashMap<String, T>) -> Self {
        JsonValue::Object(obj.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}
