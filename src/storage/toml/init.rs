use std::{
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::domain::{Backend, Project};

use super::validator::read_config;

#[derive(Clone, Serialize, Deserialize)]
pub struct TomlConfig {
    pub projects: Vec<Project>,
}

pub struct TomlBackend {
    pub path: String,
    pub config: TomlConfig,
}

impl Backend for TomlBackend {
    fn init() -> Self {
	///////////////////////////////////////////////////////////////////////
	//          Check if file exists, if yes read, if no create          //
	///////////////////////////////////////////////////////////////////////
	let dir_path = std::env::var("PCD_PATH").unwrap_or_else(|_| {
	    let home = std::env::var("HOME").unwrap();
	    format!("{}/.config/pcd", home)
	});
	let file_path = format!("{}/config.toml", dir_path);
	let file_path = Path::new(&file_path);

	if !file_path.exists() {
	    // Create the directory if it doesn't exist
	    std::fs::create_dir_all(&dir_path).expect("Failed to create config directory");

	    File::create(file_path).expect("Failed to create config file");
	}

	///////////////////////////////////////////////////////////////////////
	//          Read the file and parse it into a struct                 //
	///////////////////////////////////////////////////////////////////////
	let config_file = fs::read_to_string(file_path).expect("Failed to read toml config file");
	let config = read_config(config_file.clone()).expect("Failed to read file");

	TomlBackend {
	    path: file_path.to_string_lossy().to_string(),
	    config,
	}
    }

    fn add(&mut self, name: String, path: String) -> Result<(), crate::domain::BackendError> {
	match self.config.get_project(&name) {
	    Some(_) => {
		return Err(crate::domain::BackendError::AddProjectError(format!(
		    "Project with name '{}' already exists",
		    name
		)));
	    }
	    None => {}
	}

	let project = Project::new(name, path);
	self.config.projects.push(project);

	let toml_string = toml::to_string(&self.config).expect("Failed to serialize config");
	fs::write(&self.path, toml_string).expect("Failed to write config file");

	Ok(())
    }

    fn remove(&mut self, name: String) -> Result<(), crate::domain::BackendError> {
	match self.config.remove_project(&name) {
	    Some(_) => {}
	    None => {
		return Err(crate::domain::BackendError::RemoveProjectError(format!(
		    "Project with name '{}' does not exists",
		    name
		)));
	    }
	}

	let toml_string = toml::to_string(&self.config).expect("Failed to serialize config");
	fs::write(&self.path, toml_string).expect("Failed to write config file");

	Ok(())
    }

    fn get_projects(&self) -> Result<Vec<Project>, crate::domain::BackendError> {
	Ok(self.config.projects.clone())
    }
}

impl TomlConfig {
    fn get_project(&self, name: &str) -> Option<&Project> {
	self.projects.iter().find(|&project| project.name == name)
    }

    fn remove_project(&mut self, name: &str) -> Option<Project> {
	self.projects
	    .iter()
	    .position(|project| project.name == name)
	    .map(|index| self.projects.remove(index))
    }
}
