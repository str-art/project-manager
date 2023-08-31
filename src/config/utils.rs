use crate::utils;

use super::constants::HOME_SH_VAR;
use super::structs::Config;
use std::error::Error;
use std::{fs, path::Path};

pub fn parse_config_path(
    config_path: &str,
    home_var_name: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let home_var = match home_var_name {
        None => HOME_SH_VAR,
        Some(name) => name,
    };
    let path = Path::new(config_path);
    match path.strip_prefix(home_var) {
        Ok(path_without_prefix) => match std::env::var("HOME") {
            Err(_) => {
                let home = "/home";
                let path_string = path_without_prefix.to_str().unwrap();
                let absolute_path = format!("{}/{}", home, path_string);
                Ok(String::from(absolute_path))
            }
            Ok(home) => {
                let path_string = path_without_prefix.to_str().unwrap();
                let absolute_path = format!("{}/{}", home, path_string);
                Ok(String::from(absolute_path))
            }
        },
        Err(..) => {
            let normalized_path = utils::path::normalize_path(std::path::Path::new(config_path));
            match normalized_path.to_str() {
                Some(path_string) => Ok(String::from(path_string)),
                None => Err("Failed parsing config path")?,
            }
        }
    }
}
#[test]
fn test_path_parser() {
    let result_path = String::from("/home/test/path/to/file");
    let relative_path = "~/test/path/to/file";
    let absolute_path = "/var/path/to/file";
    let absolute_result_path = String::from(absolute_path);
    assert!(matches!(
        parse_config_path(relative_path, Some("/home")),
        Ok(result_path)
    ));
    assert!(matches!(
        parse_config_path(absolute_path, Some("/home")),
        Ok(absolute_result_path)
    ));
}
fn create_new_config(config: &Config, config_path: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(config_path);
    println!("Creating new config at path {:?}", file_path);
    match file_path.parent() {
        Some(dir_path) => match fs::create_dir_all(dir_path) {
            Ok(..) => save_existing_config(config, config_path),
            Err(err) => {
                println!("Error: {:?}", err);
                Err("Failed creating config directory")?
            }
        },
        None => save_existing_config(config, config_path),
    }
}

fn save_existing_config(config: &Config, config_path: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(config_path);
    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open(file_path)
    {
        Ok(file) => match serde_json::to_writer(file, &config) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed writing config")?,
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Err("Failed opening config file")?
        }
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_path = &config.path;
    let config_path_str = config_path.to_str().unwrap();
    println!("Saving config to path {}", config_path_str);
    match Path::exists(Path::new(config_path_str)) {
        true => save_existing_config(config, config_path_str),
        false => create_new_config(config, config_path_str),
    }
}
