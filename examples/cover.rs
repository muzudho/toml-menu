//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use chrono::{
    prelude::{DateTime, Utc},
    FixedOffset, NaiveDate, NaiveDateTime, NaiveTime,
};
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/cover.toml");

    // Read a array.
    // 配列読取。
    assert_eq!(
        doc.get_string_array_by_key("string_array"),
        Some(["a", "b", "\"c\""])
    );
}
