//! Array model.  
//! 配列モデル。  
//!
//! # Examples
//!
//! ```
//! // [ 1, 2, 3 ]
//! ```

use crate::model::{
    layer210::{DoubleQuotedString, LiteralString, LiteralValue},
    layer220::{Array, ItemValue},
};
use std::fmt;

impl Default for Array {
    fn default() -> Self {
        Array { items: Vec::new() }
    }
}
impl Array {
    pub fn push_literal_string(&mut self, m: &LiteralValue) {
        self.items.push(ItemValue::LiteralValue(m.clone()));
    }
    pub fn push_single_quote_string(&mut self, m: &LiteralString) {
        self.items.push(ItemValue::LiteralString(m.clone()));
    }
    pub fn push_double_quote_string(&mut self, m: &DoubleQuotedString) {
        self.items.push(ItemValue::DoubleQuotedString(m.clone()));
    }
    pub fn push_array(&mut self, m: &Array) {
        self.items.push(ItemValue::Array(m.clone()));
    }
}
impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "[ {} ]", buf)
    }
}
