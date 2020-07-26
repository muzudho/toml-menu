//! Syntax parser.
//! 構文パーサー。

use crate::model::InlineTableM;
use crate::syntax::{machine_state::InlineTableState, InlineTableP, KeyValueP, SyntaxParserResult};
use crate::token::{Token, TokenType};
use casual_logger::{Log, Table};

impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: InlineTableState::AfterLeftCurlyBracket,
            buffer: Some(InlineTableM::default()),
            key_value_p: None,
        }
    }
}
impl InlineTableP {
    pub fn flush(&mut self) -> Option<InlineTableM> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            InlineTableState::AfterLeftCurlyBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Key => {
                        self.key_value_p = Some(Box::new(KeyValueP::new(&token)));
                        self.state = InlineTableState::KeyValue;
                    }
                    _ => panic!(Log::fatal_t(
                        "InlineTableP#parse/AfterValue",
                        self.err_table()
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    )),
                }
            }
            InlineTableState::KeyValue => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_key_value(&child_m);
                            self.key_value_p = None;
                            self.state = InlineTableState::AfterKeyValue;
                        } else {
                            return SyntaxParserResult::Err(
                                self.err_table()
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            InlineTableState::AfterKeyValue => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                TokenType::Comma => {
                    self.state = InlineTableState::AfterLeftCurlyBracket;
                }
                TokenType::RightCurlyBracket => {
                    return SyntaxParserResult::End;
                }
                _ => panic!(Log::fatal_t(
                    "InlineTableP#parse/AfterValue",
                    self.err_table().str("token", &format!("{:?}", token))
                )),
            },
        }
        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "InlineTableP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value", &key_value_p.err_table());
        }
        t
    }
}
