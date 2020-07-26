use crate::model::SingleQuotedString;
use crate::token::Token;
use std::fmt;

impl Default for SingleQuotedString {
    fn default() -> Self {
        SingleQuotedString {
            value: String::new(),
        }
    }
}
impl SingleQuotedString {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for SingleQuotedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.value)
    }
}
