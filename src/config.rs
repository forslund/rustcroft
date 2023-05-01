use std::fs::read_to_string;
use std::path::Path;
use std::string::String;
use std::io;

use log::info;
use serde_json::{Result, Value};

extern crate xdg;


fn remove_comments(config_string: String) -> String {
    let mut ret_val = String::new();
    for line in config_string.split('\n') {
        if !line.trim().starts_with("//") {
            ret_val.push_str(line);
            ret_val.push('\n');
        }
    }
    ret_val
}


/// Read mycroft config json removing any comment lines.
fn read_config(path: &Path) -> io::Result<String> {
    let file_contents = read_to_string(path)?;
    Ok(remove_comments(file_contents))
}


/// Load mycroft configuration file from provided path
#[allow(dead_code)]
pub fn load(path: &Path) -> Result<Value> {
    let read_data = read_config(path).unwrap();
    let config: Value = serde_json::from_str(read_data.as_str())?;
    Ok(config)
}


/// Load mycroft config shipped with rustcroft
pub fn load_default() -> Result<Value>{
    let default_config_file = include_str!("../resources/mycroft.conf");
    serde_json::from_str(
        remove_comments(String::from(default_config_file)).as_str())
}


pub fn default_config_paths() -> Vec<String> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("mycroft").unwrap();
    let mut config_paths = Vec::<String>::new();
    for conf in xdg_dirs.find_config_files("mycroft.conf") {
        info!("Applying config from {}", conf.as_path().display());
        let conf_string = conf.as_path().display().to_string();
        config_paths.push(conf_string);
    }
    config_paths
}


pub struct ConfigStack {
    #[allow(dead_code)]
    files: Vec<String>,
    #[allow(dead_code)]
    configs: Vec<Value>
}

impl From<&[&str]> for ConfigStack {
    fn from(config_paths: &[&str]) -> ConfigStack {
        ConfigStack::from_slice(config_paths).unwrap()
    }
}

impl From<&Vec<String>> for ConfigStack {
    fn from(config_paths: &Vec<String>) -> ConfigStack {
        ConfigStack::from_vec(config_paths).unwrap()
    }
}

impl From<Vec<String>> for ConfigStack {
    fn from(config_paths: Vec<String>) -> ConfigStack {
        ConfigStack::from_vec(&config_paths).unwrap()
    }
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

    /// Create config stack from Vector of paths
    pub fn from_vec(config_paths: &Vec<String>) -> Result<ConfigStack> {
        let mut path_vec = Vec::<String>::new();
        let mut config_vec = Vec::<Value>::new();
        for config in config_paths {
            path_vec.push(config.clone());
            config_vec.push(load(Path::new(config.as_str())).unwrap());
        }
        Ok(ConfigStack {files: path_vec, configs: config_vec})
    }

    /// Create config stack from default config files (XDG locations)
    /// and the config packed with the rustcroft library
    pub fn from_default() -> Result<ConfigStack> {
        let config_paths = default_config_paths();
        let mut stack = ConfigStack::from(&config_paths);
        // Add default config provided by rustcroft
        stack.files.push(String::from("default"));
        stack.configs.push(load_default().unwrap());
        Ok(stack)
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
                    Some(res) => field = res,
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
