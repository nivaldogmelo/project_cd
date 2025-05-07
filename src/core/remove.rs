use inquire::InquireError;

use crate::domain::Project;

pub fn remove(projects: Vec<Project>) -> Result<String, String> {
    let options = projects.iter().collect();

    let ans: Result<Project, InquireError> = inquire::Select::new("Select a project:", options)
	.prompt()
	.cloned();

    match ans {
	Ok(project) => Ok(project.name().to_string()),
	Err(e) => Err(format!("Error: {}", e)),
    }
}
