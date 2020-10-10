//! Double quoted string syntax parser.  
//! 二重引用符文字列構文パーサー。  

use crate::model::layer10::DoubleQuotedString;
use crate::parser::syntax::{layer10::DoubleQuotedStringP, ResultSP};
use crate::token::{Token, TokenType};
use casual_logger::Table;

impl DoubleQuotedStringP {
    pub fn flush(&mut self) -> Option<DoubleQuotedString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        DoubleQuotedStringP {
            buffer: Some(DoubleQuotedString::default()),
        }
    }
    /// # Returns
    ///
    /// * `ResultSP` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> ResultSP {
        match token.type_ {
            // `"`
            TokenType::DoubleQuotation => {
                // End of syntax.
                // 構文の終わり。
                return ResultSP::End;
            }
            _ => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token);
            }
        }
        ResultSP::Ongoing
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
