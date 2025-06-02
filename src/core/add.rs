use std::path::PathBuf;

use inquire::Text;

use crate::domain::Project;

pub fn add(path: PathBuf) -> Result<Project, String> {
    let default_name = path
	.file_name()
	.and_then(|name| name.to_str())
	.unwrap_or("new_project")
	.to_string();

    let input_msg = format!(
	"What's gonna be the project name? (default: {})",
	default_name
    );
    let name = Text::new(&input_msg).prompt();

    match name {
	Ok(name) => {
	    let mut project_name = String::new();
	    if name.is_empty() {
		project_name = default_name;
	    } else {
		project_name = name;
	    }

	    println!("Project name: {}", project_name);
	    let project = Project::new(project_name, path.clone());
	    Ok(project)
	}
	Err(e) => {
	    eprintln!("Error: {}", e);
	    Err(format!("Error: {}", e))
	}
    }
}
