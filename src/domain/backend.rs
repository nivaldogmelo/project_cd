use std::path::PathBuf;

use thiserror::Error;

use super::Project;

pub trait Backend {
    fn init() -> Self;
    fn add(&mut self, name: String, path: PathBuf) -> Result<(), BackendError>;
    fn remove(&mut self, name: String) -> Result<(), BackendError>;
    fn get_projects(&self) -> Result<Vec<Project>, BackendError>;
}

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Failed to add project: {0}")]
    AddProjectError(String),
    #[error("Failed to remove project: {0}")]
    RemoveProjectError(String),
    #[error("Failed to get projects: {0}")]
    GetProjectsError(String),
}
