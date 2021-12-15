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
#[allow(dead_code)]
pub fn load(path: &Path) -> Result<Value> {
    let read_data = read_config(&path).unwrap();
    let config: Value = serde_json::from_str(read_data.as_str())?;
    Ok(config)
}


pub struct ConfigStack {
    #[allow(dead_code)]
    files: Vec<String>,
    #[allow(dead_code)]
    configs: Vec<Value>
}

impl ConfigStack {
    #[allow(dead_code)]
    /// Create config stack from slice of paths
    pub fn from_slice(config_paths: &[&str]) -> Result<ConfigStack> {
        let mut path_vec = Vec::<String>::new();
        let mut config_vec = Vec::<Value>::new();
        for config in config_paths {
            path_vec.push(String::from(*config));
            config_vec.push(load(Path::new(*config)).unwrap());
        }
        Ok(ConfigStack {files: path_vec, configs: config_vec})
    }

    #[allow(dead_code)]
    /// Get a value from the first config that has a matching value
    pub fn get(&self, key_sequence: &[&str])
            -> std::result::Result<Value, &str> {
        let mut field;
        let mut found = false;
        let mut wrong_type = false;
        for conf in self.configs.iter() {
            field = conf;
            for key in key_sequence.iter() {
                found = true;
                match field.get(key) {
                    Some(res) => field = &res,
                    _ => found = false
                }

                let is_last_item = key_sequence.last().unwrap() == key;
                wrong_type = found && (!is_last_item && !field.is_object());
                if !found || wrong_type {
                    break;
                }
            }
            if found && !wrong_type {
                return Ok((*field).clone())
            }
        }
        Err("Key not found")
    }
}
