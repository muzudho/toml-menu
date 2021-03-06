pub mod expression;
pub mod header_of_array_of_table;
pub mod header_of_table;

use crate::model::{
    layer110::Token,
    layer210::{Comment, Ws},
    layer225::Keyval,
};

/// WIP.  
#[derive(Clone)]
pub struct HeaderOfArrayOfTable {
    pub tokens: Vec<Token>,
}

/// Either a Empty-line, Comment, Key Value, Table or a Array-of-table.  
/// 空行、コメント、キー値、テーブル、テーブルの配列のいずれかです。  
#[derive(Clone)]
pub enum Expression {
    HeaderOfArrayOfTable(HeaderOfArrayOfTable),
    EmptyLine(Ws, Option<Comment>),
    Keyval(Ws, Keyval, Ws, Option<Comment>),
    HeaderOfTable(HeaderOfTable),
}

/// WIP.  
#[derive(Clone)]
pub struct HeaderOfTable {
    pub tokens: Vec<Token>,
}
