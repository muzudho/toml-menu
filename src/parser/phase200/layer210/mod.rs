pub mod basic_string_p;
pub mod comment_p;
pub mod date_time_p;
pub mod escape_sequence_p;
pub mod header_p_of_array_of_table;
pub mod header_p_of_table;
pub mod key_p;
pub mod keyval_sep_p;
pub mod literal_string_p;
pub mod literal_value_p;
pub mod non_ascii_p;
pub mod non_eol_p;
pub mod positional_numeral_string_p;
pub mod ws_p;
pub mod wschar_p;

use crate::model::{
    layer210::{BasicString, Comment, Key, LiteralString, LiteralValue, Ws, Wschar},
    layer230::{HeaderOfArrayOfTable, HeaderOfTable},
};
use crate::parser::phase200::layer210::{
    basic_string_p::State as BasicStringState, comment_p::State as CommentState,
    date_time_p::State as DateTimeState, escape_sequence_p::State as EscapeSequenceState,
    keyval_sep_p::State as KeyvalSepPState, literal_string_p::State as LiteralStringState,
    literal_value_p::State as LiteralValueState, ws_p::State as WsPState,
    wschar_p::State as WscharState,
};
use crate::parser::phase200::Token;
use casual_logger::Table as LogTable;

/// Double quoted string syntax parser.  
/// 二重引用符文字列構文パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct BasicStringP {
    buffer: Option<BasicString>,
    state: BasicStringState,
    escape_sequence_p: Option<EscapeSequenceP>,
}

/// Comment parser.  
/// コメント・パーサー。  
///
/// Example: `# comment`.  
#[derive(Clone)]
pub struct CommentP {
    product: Comment,
    state: CommentState,
}

/// Date time parser.  
/// 年月日日付パーサー。  
///
/// Example: `# comment`.  
#[derive(Clone)]
pub struct DateTimeP {
    buffer: Vec<Token>,
    state: DateTimeState,
}

/// Escape sequence parser.  
/// エスケープ・シーケンス・パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct EscapeSequenceP {
    positional_numeral_string_p: Option<PositionalNumeralStringP>,
    buffer: Vec<Token>,
    state: EscapeSequenceState,
    string_buffer: String,
}

/// Header of array of table syntax parser.  
/// テーブル配列ヘッダー構文パーサー。  
///
/// Example: `[[value]]`.  
#[derive(Clone)]
pub struct HeaderPOfArrayOfTable {
    buffer: Option<HeaderOfArrayOfTable>,
}

/// Header of table syntax parser.  
/// テーブル・ヘッダー構文パーサー。  
///
/// Example: `[value]`.  
#[derive(Clone)]
pub struct HeaderPOfTable {
    buffer: Option<HeaderOfTable>,
}

/// Non ascii parser.  
/// 非ASCIIパーサー。  
#[derive(Clone)]
pub struct NonAsciiP {}

/// Non end-of-line parser.  
/// 非行末パーサー。  
#[derive(Clone)]
pub struct NonEolP {}

/// Hex string parser.  
/// 16進文字列パーサー。  
///
/// Example: `01Ab23cD`.  
#[derive(Clone)]
pub struct PositionalNumeralStringP {
    buffer: Vec<Token>,
    /// `0b`, `0o`, '', `0x`.
    prefix: String,
    string_buffer: String,
    /// 桁数をぴったり指定したければこれ。でなければ 0。
    expected_digits: usize,
}

/// Keyval-separator parser.  
/// キー値仕切りパーサー。  
///
/// Example: ` = `.  
#[derive(Clone)]
pub struct KeyvalSepP {
    state: KeyvalSepPState,
    ws1: Ws,
    ws2: Ws,
}

/// Key parser.  
/// キー・パーサー。  
///
/// Example: `abc`.  
#[derive(Clone)]
pub struct KeyP {
    buffer: Option<Key>,
}

/// Result of syntax parser.  
/// 構文パーサーの結果。  
pub enum PResult {
    /// End of syntax.
    End,
    // EndCarryOver(Token),
    Ongoing,
    /// Error.
    Err(LogTable),
}

/// Literal string syntax parser.  
/// 単一引用符文字列構文パーサー。  
///
/// Example: `'value'`.  
#[derive(Clone)]
pub struct LiteralStringP {
    buffer: Option<LiteralString>,
    state: LiteralStringState,
}

/// Literal value syntax parser.  
/// リテラル値構文パーサー。  
///
/// Example: `abc`.  
#[derive(Clone)]
pub struct LiteralValueP {
    date_time_p: Option<DateTimeP>,
    positional_numeral_string_p: Option<PositionalNumeralStringP>,
    buffer: Option<LiteralValue>,
    state: LiteralValueState,
}

/// Comment parser.  
/// コメント・パーサー。  
///
/// Example: `# comment`.  
#[derive(Clone)]
pub struct WsP {
    ws: Ws,
    state: WsPState,
    wschar_p: Option<WscharP>,
}

/// White space character parser.  
/// 空白文字パーサー。  
#[derive(Clone)]
pub struct WscharP {
    buffer: Option<Wschar>,
    state: WscharState,
}
