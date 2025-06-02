use project_cd::{
    clap_cli::ClapCli,
    core::{add, get_project, remove, search},
    domain::{Backend, Cli, CliActions},
    storage,
};

fn main() -> Result<(), std::io::Error> {
    // Initialize the backend
    let mut backend = storage::TomlBackend::init();

    let cli = ClapCli::new();
    let parsed = cli.parse();

    match parsed {
	CliActions::Search => {
	    let path = search(backend.projects.clone());
	    if let Err(e) = path {
		println!("Error: {}", e);
		return Err(std::io::Error::new(
		    std::io::ErrorKind::Other,
		    "Error searching for project",
		));
	    }
	    let path = path.unwrap();
	    print!("{}", path.display());
	}
	CliActions::Add(project) => {
	    if let Ok(path) = project {
		let project = add(path.clone()).expect("Error adding project");
		backend
		    .add(project.name.clone(), project.path.clone())
		    .expect("Error adding project");
		println!("Adding project directory: {:?}", path.display());
	    } else {
		println!("No project directory provided");
	    }
	}
	CliActions::Remove(project) => {
	    let name = match project {
		Some(name) => name,
		None => remove(backend.projects.clone()).expect("Error removing project"),
	    };

	    backend
		.remove(name.clone())
		.expect("Error removing project");
	}
	CliActions::Chdir(project) => {
	    let path = get_project(project, backend.projects.clone());
	    if let Err(e) = path {
		println!("Error: {}", e);
		return Err(std::io::Error::new(
		    std::io::ErrorKind::Other,
		    "Error searching for project",
		));
	    }
	    let path = path.unwrap();
	    print!("{}", path.display());
	}
	_ => {}
    }

    Ok(())
}
