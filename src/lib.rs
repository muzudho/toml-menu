//! TOML parser.  
//! トムル解析器。  
//!
//! `cargo run --example main`

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod model;
mod parser;

use crate::model::layer310::Document;
use crate::parser::{
    phase100::lexical_parser::LexicalParser,
    phase200::{layer210::PResult, layer310::document_line_scanner::DocumentLineScanner},
};
use casual_logger::{ArrayOfTable, Log, Table};
use regex::Regex;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

lazy_static! {
    /// Without dot.
    /// ドットは含みません。
    pub static ref RE_KEY: Mutex<Regex> = Mutex::new(Regex::new(r"[A-Za-z0-9_-]$").unwrap());
}

/// TOML.  
/// トムル。  
pub struct Toml {}
impl Toml {
    /// Line scan.
    /// 行走査。
    pub fn from_file(path: &str) -> Document {
        let mut error_tables = Vec::<Table>::new();
        let mut output_document = Document::default();
        match File::open(path) {
            Ok(file) => {
                for (i, line) in BufReader::new(file).lines().enumerate() {
                    let row_number = i + 1;
                    let line = match line {
                        Ok(line) => line,
                        Err(why) => panic!(Log::fatal(&format!("{}", why))),
                    };
                    // Log::trace(&format!("from_file/line=|{}|", line));
                    let mut lexical_p = LexicalParser::new(row_number);
                    lexical_p.parse_line(&line);

                    let mut document_line_scanner = DocumentLineScanner::default();
                    match document_line_scanner
                        .scan_line(&lexical_p.product(), &mut output_document)
                    {
                        PResult::End => {} // Ignored it.
                        PResult::Err(table) => {
                            error_tables.push(
                                Table::default()
                                    .str("via", "lib.rs.65.")
                                    .int(
                                        "row_number",
                                        if let Ok(n) = row_number.try_into() {
                                            n
                                        } else {
                                            -1
                                        },
                                    )
                                    .str("line", &format!("{}", line))
                                    .str("token_line", &format!("{:?}", lexical_p))
                                    .sub_t("table", &table)
                                    .sub_t(
                                        "document_line_scanner",
                                        &document_line_scanner.log_snapshot(),
                                    )
                                    .clone(),
                            );
                        }
                        PResult::Ongoing => {} // Ignored it.
                    }
                }
            }
            Err(why) => panic!("{}", why),
        }

        if !error_tables.is_empty() {
            let mut error_aot = ArrayOfTable::default();
            for err_tbl in error_tables {
                error_aot.table(&err_tbl);
            }
            Log::error_t(
                "List if error exists.",
                Table::default().sub_aot("error_aot", &error_aot),
            );
        }

        output_document
    }
}
