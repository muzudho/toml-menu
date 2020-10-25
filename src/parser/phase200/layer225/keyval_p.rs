//! Key value syntax parser.  
//! キー値構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // key = right_value
//! ```

use crate::model::{layer110::TokenType, layer225::Keyval};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::{
    layer210::{KeyP, PResult},
    layer225::{KeyvalP, RightValueP},
};
use casual_logger::Table as LogTable;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug)]
pub enum State {
    AfterEquals,
    // After key.
    // キーの後。
    BeforeEqual,
    End,
    First,
    RightValue,
}

impl KeyvalP {
    pub fn new() -> Self {
        KeyvalP {
            key_buffer: None,
            right_value_buffer: None,
            key_p: Some(KeyP::default()),
            right_value_p: None,
            state: State::First,
        }
    }

    pub fn flush(&mut self) -> Option<Keyval> {
        let m = if let Some(key) = &self.key_buffer {
            if let Some(right_value) = &self.right_value_buffer {
                Some(Keyval::new(&key, &right_value))
            } else {
                panic!("keyval_p.rs.53.")
            }
        } else {
            panic!("keyval_p.rs.56.")
        };
        self.key_buffer = None;
        self.right_value_buffer = None;
        m
    }

    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match self.state {
            // After `=`.
            State::AfterEquals => {
                self.right_value_p = Some(RightValueP::default());
                self.state = State::RightValue;
            }
            // After key.
            State::BeforeEqual => {
                match token0.type_ {
                    TokenType::WhiteSpaceString => {} //Ignored it.
                    // `=`.
                    TokenType::Equals => {
                        self.state = State::AfterEquals;
                    }
                    _ => return error(&mut self.log(), &tokens, "keyval.rs.65."),
                }
            }
            State::First => {
                match token0.type_ {
                    TokenType::WhiteSpaceString => {} //Ignored it.
                    TokenType::AbChar
                    | TokenType::NumChar
                    | TokenType::Hyphen
                    | TokenType::Underscore => {
                        let p = self.key_p.as_mut().unwrap();
                        match p.parse(&tokens) {
                            PResult::End => {
                                if let Some(child_m) = p.flush() {
                                    self.key_buffer = Some(child_m);
                                    self.key_p = None;
                                    self.state = State::BeforeEqual;
                                } else {
                                    return error(&mut self.log(), &tokens, "keyval.rs.84.");
                                }
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    &tokens,
                                    "keyval.rs.84.",
                                );
                            }
                            PResult::Ongoing => {}
                        }
                    }
                    _ => return error(&mut self.log(), &tokens, "keyval.rs.65."),
                }
            }
            // After `=`.
            State::RightValue => {
                let p = self.right_value_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.right_value_buffer = Some(child_m);
                            self.right_value_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "keyval.rs.84.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), &tokens, "keyval.rs.88.");
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => return error(&mut self.log(), &tokens, "keyval.rs.93."),
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(m) = &self.key_buffer {
            t.str("key_buffer", &m.to_string());
        }
        if let Some(m) = &self.right_value_buffer {
            t.str("right_value_buffer", &m.to_string());
        }
        if let Some(p) = &self.right_value_p {
            t.sub_t("right_value_p", &p.log());
        }
        t
    }
}