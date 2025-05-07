use inquire::InquireError;

use crate::domain::Project;

pub fn search(projects: Vec<Project>) -> Result<String, String> {
    let options = projects.iter().collect();

    let ans: Result<Project, InquireError> = inquire::Select::new("Select a project:", options)
	.prompt()
	.cloned();

    match ans {
	Ok(project) => Ok(project.path().to_string()),
	Err(e) => Err(format!("Error: {}", e)),
    }
}

pub fn get_project(project_name: String, projects: Vec<Project>) -> Result<String, String> {
    let project = projects
	.iter()
	.find(|p| *p.name() == project_name)
	.ok_or_else(|| format!("Project {} not found", project_name))?;

    let project_path = project.path().to_string();
    Ok(project_path)
}
