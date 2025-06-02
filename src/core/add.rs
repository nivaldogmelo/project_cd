use std::path::PathBuf;

use inquire::Text;

use crate::domain::Project;

pub fn add(path: PathBuf) -> Result<Project, String> {
    let name = Text::new("What's gonna be the project name?").prompt();

    match name {
	Ok(name) => {
	    println!("Project name: {}", name);
	    let project = Project::new(name.clone(), path.clone());
	    Ok(project)
	}
	Err(e) => {
	    eprintln!("Error: {}", e);
	    Err(format!("Error: {}", e))
	}
    }
}
