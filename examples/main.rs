//! WIP.
//! Test config file.
//! 設定ファイルのテスト。
//!
//! `cargo run --example main`

use toml_menu::tokenizer::Document;

fn main() {
    let _doc = Document::from_file("./casual-logger.type.toml");
}