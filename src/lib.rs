//! `rainstash-rs` is a simple library for parsing Rainstash JSON data into simple structs.
//!
//! Original Rainstash website repository can be found at:
//! [Rainstash Github](https://github.com/Fustran/rainstash)
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! rainstash = "0.1.0"
//! ```
//!
//! and to the top of your `main.rs`:
//!
//! ```rs
//! extern crate rainstash;
//! ```
#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(doc_markdown, inline_always)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod parser;
pub mod utils;
pub mod matching;

#[cfg(test)]
mod tests {
    use utils::error::RainstashError;

    #[test]
    fn parser_test() -> Result<(), RainstashError> {
        use parser::json::parse_items_from_file;
        use std::path::Path;

        let parsed = parse_items_from_file(Path::new("tests/test_object.json"))?;
        let object = parsed.get("Test_Item").unwrap();

        Ok(assert_eq!(object.name, "Test Item"))
    }
}
