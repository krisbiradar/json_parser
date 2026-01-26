use std::collections::HashMap;

use crate::core::token::Token;

pub struct FSM {
    pub current_sequence: Vec<u8>,
    pub total_bytes_consumed: usize,
    pub total_bytes_to_consume: usize,
    pub current_state: FSMState,
    pub all_tokens: HashMap<usize, Token>,
    pub current_token_idx: usize,
}

impl FSM {
    pub fn new(bytes_to_consume: usize) -> Self {
        Self {
            current_sequence: Vec::new(),
            current_state: FSMState::Start,
            all_tokens: HashMap::new(),
            total_bytes_consumed: 0,
            total_bytes_to_consume: bytes_to_consume,
            current_token_idx: 0,
        }
    }
    pub fn last_token(&self) -> Option<Token> {
        if (self.current_token_idx == 0) {
            return None;
        } else {
            return Some(self.all_tokens[&(self.current_token_idx - 1)].clone());
        }
    }
    pub fn current_token(&self) -> Option<Token> {
        if (self.current_token_idx == 0) {
            return None;
        } else {
            return Some(self.all_tokens[&(self.current_token_idx)].clone());
        }
    }
    pub fn processed(&self) -> bool {
        self.total_bytes_consumed == self.total_bytes_to_consume
    }
}
pub enum FSMState {
    Start,
    EnKeyString,
    EnValue,
    EnStringValue,
    EnNumberValue,
    End,
}
