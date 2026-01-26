use std::collections::HashMap;

use crate::core::token::Token;

pub struct FSM {
    pub current_sequence: Vec<u8>,
    pub current_quote_state: FSMQuoteState,
    pub current_state: FSMState,
    pub all_tokens: HashMap<usize, Token>,
    pub current_token_idx: usize,
}

impl FSM {
    pub fn new() -> Self {
        Self {
            current_sequence: Vec::new(),
            current_state: FSMState::Start,
            current_quote_state: FSMQuoteState::KeyEnd,
            all_tokens: HashMap::new(),
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
    
}
pub enum FSMQuoteState {
    KeyStart,
    KeyEnd,
    ValueStart,
    ValueEnd,
}
pub enum FSMState {
    Start,
    Enkey,
    EnValue,
    End,
}
