// src/utils/file.rs

use crate::utils::error::{HubSpotError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_properties<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>> {
    let file = File::open(file_path).map_err(|e| HubSpotError::FileError(e.to_string()))?;
    let reader = BufReader::new(file);
    let properties: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    Ok(properties)
}
