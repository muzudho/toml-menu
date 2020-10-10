//! DocumentElement model.  
//! 縦幅のある行 モデル。  

use crate::model::{
    layer10::Comment,
    layer20::KeyValue,
    layer30::{ArrayOfTable, DocumentElement, Table},
};
use std::fmt;

impl DocumentElement {
    pub fn from_array_of_table(m: &ArrayOfTable) -> Self {
        DocumentElement::ArrayOfTable(m.clone())
    }
    pub fn from_comment(m: &Comment) -> Self {
        DocumentElement::Comment(m.clone())
    }
    pub fn from_key_value(m: &KeyValue) -> Self {
        DocumentElement::KeyValue(m.clone())
    }
    pub fn from_table(m: &Table) -> Self {
        DocumentElement::Table(m.clone())
    }
}
impl fmt::Debug for DocumentElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocumentElement::ArrayOfTable(m) => write!(f, "{}", format!("{:?}", m)),
            DocumentElement::Comment(m) => write!(f, "{}", format!("{:?}", m)),
            DocumentElement::EmptyLine => write!(f, ""),
            DocumentElement::KeyValue(m) => write!(f, "{}", format!("{:?}", m)),
            DocumentElement::Table(m) => write!(f, "{}", format!("{:?}", m)),
        }
    }
}
