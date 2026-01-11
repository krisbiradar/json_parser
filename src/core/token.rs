use crate::core::tokentype::TokenType;
use std::any::Any;

pub struct Token {
    token_type: TokenType,
    value: Option<Box<dyn Any>>,
    start_pos: usize,
    end_pos: Option<usize>,
    token_idx: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start_pos: usize, token_idx: usize) -> Self {
        Self {
            token_type,
            value: None,
            start_pos,
            end_pos: None,
            token_idx,
        }
    }

    pub fn with_value(
        token_type: TokenType,
        start_pos: usize,
        token_idx: usize,
        value: Box<dyn Any>,
    ) -> Self {
        Self {
            token_type,
            value: Some(value),
            start_pos,
            end_pos: None,
            token_idx,
        }
    }

    pub fn with_end_pos(mut self, end_pos: usize) -> Self {
        self.end_pos = Some(end_pos);
        self
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn start_pos(&self) -> usize {
        self.start_pos
    }

    pub fn end_pos(&self) -> Option<usize> {
        self.end_pos
    }

    pub fn token_idx(&self) -> usize {
        self.token_idx
    }

    pub fn value(&self) -> Option<&Box<dyn Any>> {
        self.value.as_ref()
    }

    pub fn as_string(&self) -> Option<&String> {
        self.value.as_ref()?.downcast_ref::<String>()
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.value.as_ref()?.downcast_ref::<f64>().copied()
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.value.as_ref()?.downcast_ref::<bool>().copied()
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = if let Some(ref v) = self.value {
            if let Some(s) = v.downcast_ref::<String>() {
                format!("\"{}\"", s)
            } else if let Some(n) = v.downcast_ref::<f64>() {
                format!("{}", n)
            } else if let Some(b) = v.downcast_ref::<bool>() {
                format!("{}", b)
            } else {
                "?".to_string()
            }
        } else {
            "None".to_string()
        };

        f.debug_struct("Token")
            .field("type", &self.token_type)
            .field("value", &value_str)
            .field("start_pos", &self.start_pos)
            .field("token_idx", &self.token_idx)
            .finish()
    }
}
