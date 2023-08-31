use super::constants::CONFIG_NOT_FOUND;
use super::utils::{parse_config_path, save_config};
use crate::utils::path::normalize_path;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: PathBuf,
    pub projects: Vec<Project>,
}

impl Config {
    pub fn save(&self) -> Result<&Self, Box<dyn Error>> {
        match save_config(self) {
            Ok(()) => Ok(self),
            Err(err) => Err(err),
        }
    }
    pub fn rename_project(&mut self, old_name: &str, new_name: &str) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.get_project(new_name) {
            Err(format!("Project with name {new_name} already exists"))?
        }
        match self.get_project(old_name) {
            Some(project) => {
                project.set_name(String::from(new_name));
                Ok(())
            }
            None => Err(format!("Project {old_name} not foun"))?,
        }
    }
    pub fn remove_project(&mut self, name: &str) {
        if let Some(pos) = self.projects.iter().position(|p| p.name == name) {
            self.projects.remove(pos);
        };
    }
    pub fn add_project(&mut self, name: &str, project_path: &str) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.get_project(name) {
            Err(format!("Project with name {name} already exists"))?
        }
        let path = normalize_path(Path::new(project_path));
        println!("New project path is {}", path.display());
        let project = Project {
            name: String::from(name),
            path,
        };
        self.projects.push(project);
        Ok(())
    }
    pub fn get_project(&mut self, name: &str) -> Option<&mut Project> {
        match self.projects.iter().position(|p| p.name == name) {
            Some(pos) => Some(&mut self.projects[pos]),
            None => None,
        }
    }
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let file_path_string = parse_config_path(path, None).unwrap();
        let path_buf = std::path::Path::new(&file_path_string);
        let is_created = std::path::Path::exists(path_buf);
        if !is_created {
            return Err(CONFIG_NOT_FOUND)?;
        }
        match fs::read_to_string(path_buf) {
            Ok(content) => {
                let config: Config = serde_json::from_str(&content)?;
                Ok(config)
            }
            Err(err) => {
                println!("Error while reading file, {:?}", err);
                Err("Failed reading config")?
            }
        }
    }
    pub fn create(path: &str) -> Result<Self, Box<dyn Error>> {
        match parse_config_path(path, None) {
            Ok(config_path) => {
                let config = Config {
                    path: PathBuf::from(config_path),
                    projects: Vec::new(),
                };
                match save_config(&config) {
                    Ok(_) => Ok(config),

                    Err(err) => {
                        println!("Error: {:?}", err);
                        Err("Failed saving file")?
                    }
                }
            }

            Err(_) => Err("Failed parsing config path")?,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
}
impl Project {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
