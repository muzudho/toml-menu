//! Hex string parser.  
//! 16進文字列パーサー。  

use crate::model::layer110::{Token, TokenType};
use crate::parser::phase200::error2;
use crate::parser::phase200::layer210::{HexStringP, PResult};
use crate::parser::phase200::LookAheadTokens;
use casual_logger::Table;

impl Default for HexStringP {
    fn default() -> Self {
        HexStringP {
            buffer: Vec::new(),
            string_buffer: String::new(),
            expected_digits: 0,
        }
    }
}
impl HexStringP {
    pub fn set_expected_digits(&mut self, val: usize) -> &mut Self {
        self.expected_digits = val;
        self
    }
    pub fn flush(&mut self) -> Vec<Token> {
        let m = self.buffer.clone();
        self.buffer.clear();
        m
    }
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(
        &mut self,
        tokens_old: (Option<&Token>, Option<&Token>, Option<&Token>),
    ) -> PResult {
        let tokens = LookAheadTokens::from_old(tokens_old);
        let token0 = tokens.current.as_ref().unwrap();

        match token0.type_ {
            TokenType::NumeralString | TokenType::AlphabetCharacter | TokenType::AlphabetString => {
                let s = token0.to_string();
                let current_expected = self.expected_digits - self.string_buffer.len();
                let (addition, overflow) = if current_expected < s.len() {
                    (
                        s[0..current_expected].to_string(),
                        s[current_expected..].to_string(),
                    )
                } else {
                    (s[0..].to_string(), "".to_string())
                };

                self.string_buffer.push_str(&addition);

                // Filled.
                // 満ちたなら。
                if self.expected_digits <= self.string_buffer.len() {
                    /*
                    println!(
                        // "[trace56={}][self.expected_digits={}][self.string_buffer.len()={}]",
                        self.string_buffer,
                        self.expected_digits,
                        self.string_buffer.len()
                    );
                    */
                    self.buffer.push(Token::new(
                        token0.column_number,
                        &self.string_buffer,
                        TokenType::AlphabetString, // TODO Alphabet or Number String
                    ));
                    return PResult::End;
                }

                if 0 < overflow.len() {
                    self.buffer.push(Token::new(
                        token0.column_number,
                        &overflow.to_string(),
                        TokenType::AlphabetString, // TODO Alphabet or Number String
                    ));
                }
            }
            _ => {
                return error2(&mut self.log(), &tokens, "hex_string_p.rs.179.");
            }
        }

        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();

        let mut buf = String::new();
        for token in &self.buffer {
            buf.push_str(&token.to_string());
        }

        t.str("value", &buf);
        t
    }
}