//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenType};
use crate::syntax::SyntaxParserResult;
use casual_logger::Table;

/// `'value'`.
pub struct SingleQuotedStringParser {
    value: String,
}
impl SingleQuotedStringParser {
    pub fn new() -> Self {
        SingleQuotedStringParser {
            value: String::new(),
        }
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match token.type_ {
            TokenType::SingleQuotation => {
                // End of syntax.
                // 構文の終わり。
                return SyntaxParserResult::Ok(true);
            }
            _ => {
                self.value.push_str(&token.value);
            }
        }
        SyntaxParserResult::Ok(false)
    }
    pub fn log(&self) -> Table {
        Table::default().str("value", &self.value).clone()
    }
}