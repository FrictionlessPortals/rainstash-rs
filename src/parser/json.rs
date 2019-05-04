//! The Rainstash Manifest JSON Parser.
//!
//! The parser offers ways to load the manifest from string literals or
//! from a file.
//!
//! The parser also offers ways to update from Rainstash or load from a file
//! cached from the last update, removing the need to download the file every function
//! call.
use crate::{parser::manifest::RiskItem, utils::error::RainstashError};
use log::info;
use reqwest::get;
use serde_json::{from_reader, from_value, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Hard coded Rainstash Vanilla Manifest URL. User can also pass a custom one into the function call when updating.
static VANILLA_URL: &'static str =
    "https://fustran.github.io/rainstash/items/vanilla_items/itemManifest.json";

/// Download and or update the Rainstash local manifest cache.
/// The user can pass a custom rainstash_url or supply "None" for the function to use
/// the hardcoded URL string for the manifest.
pub fn update_rainstash_cache<T: AsRef<Path>>(
    rainstash_url: Option<&str>,
    path: T,
) -> Result<(), RainstashError> {
    let url = match rainstash_url {
        Some(u) => u,
        None => VANILLA_URL,
    };

    info!("[!] Updating database: GET {}", url);
    let response = get(url)?.text()?;

    info!("[!] Saved updated file to: {:?}", path.as_ref().to_str());
    let mut file = File::create(path)?;
    file.write_all(response.as_bytes())?;

    Ok(())
}

/// Simple function to parse the items object from a locally downloaded manifest.
/// Returns a HashMap of the item object name and the RiskItem.
pub fn parse_items_from_file<T: AsRef<Path>>(
    path: T,
) -> Result<HashMap<String, RiskItem>, RainstashError> {
    info!("[!] Opening manifest file and parsing items object.");

    let file: Value = from_reader(File::open(path)?)?;
    let parsed: HashMap<String, RiskItem> = from_value(
        file.get("items")
            .expect("[!] Could not parse items section of JSON.")
            .to_owned(),
    )?;

    Ok(parsed)
}

/// Simple function to parse the classInfo object from a locally downloaded manifest.
/// Returns a HashMap of the class object name and the color as a String.
pub fn parse_class_info_from_file<T: AsRef<Path>>(
    path: T,
) -> Result<HashMap<String, HashMap<String, String>>, RainstashError> {
    info!("[!] Opening manifest file and parsing classInfo object.");

    let file: Value = from_reader(File::open(path)?)?;
    let parsed: HashMap<String, HashMap<String, String>> = from_value(
        file.get("classInfo")
            .expect("[!] Could not parse classInfo section of JSON.")
            .to_owned(),
    )?;

    Ok(parsed)
}

/// Simple function to parse the commandSort object from a locally downloaded manifest.
/// Returns a Vector of item name strings in order.
pub fn parse_command_sort_from_file<T: AsRef<Path>>(
    path: T,
) -> Result<Vec<String>, RainstashError> {
    info!("[!] Opening manifest file and parsing commandSort object.");

    let file: Value = from_reader(File::open(path)?)?;
    let parsed: Vec<String> = from_value(
        file.get("commandSort")
            .expect("[!] Could not parse commandSort section of JSON.")
            .to_owned(),
    )?;

    Ok(parsed)
}
