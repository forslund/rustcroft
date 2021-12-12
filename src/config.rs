use std::fs::read_to_string;
use std::path::Path;
use std::string::String;
use std::io;

use serde_json::{Result, Value};

/// Read mycroft config json removing any comment lines.
fn read_config(path: &Path) -> io::Result<String> {
    let s = read_to_string(path)?;
    let mut ret_val = String::new();
    for line in s.split('\n') {
        if !line.trim().starts_with("//") {
            ret_val.push_str(line);
            ret_val.push('\n');
        }
    }
    Ok(ret_val)
}


/// Load mycroft configuration file from provided path
pub fn load(path: &Path) -> Result<Value> {
    let read_data = read_config(&path).unwrap();
    let config: Value = serde_json::from_str(read_data.as_str())?;
    Ok(config)
}
