pub mod toml_document;

use crate::model::layer230::DocumentElement;

/// It has multiple `document_element`.  
/// 複数の `縦幅を持つ行` を持ちます。  
#[derive(Clone)]
pub struct TomlDocument {
    /// Line with height.
    /// 縦幅を持つ行。
    pub elements: Vec<DocumentElement>,
}
