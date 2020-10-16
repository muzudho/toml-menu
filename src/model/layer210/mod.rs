pub mod basic_string;
pub mod comment;
pub mod literal_string;
pub mod literal_value;

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct Comment {
    pub value: String,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct DoubleQuotedString {
    pub value: String,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct LiteralValue {
    pub value: String,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct LiteralString {
    pub value: String,
}
