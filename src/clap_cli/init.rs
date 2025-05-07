use clap::{Arg, Command};

use crate::{
    clap_cli::wrapper::install_wrapper,
    domain::{Cli, CliActions},
};

use super::add::add;

pub struct ClapCli {
    command: Command,
}

impl Cli for ClapCli {
    fn new() -> Self {
	let command = Command::new("pcd")
	    .about("project cd")
	    .version("0.0.1")
	    .arg(Arg::new("project_name"))
	    .subcommand(
		Command::new("add")
		    .short_flag('a')
		    .long_flag("add")
		    .about("Add new project")
		    .arg(Arg::new("project_dir")),
	    )
	    .subcommand(
		Command::new("remove")
		    .short_flag('r')
		    .long_flag("remove")
		    .about("Remove project")
		    .arg(Arg::new("project_name")),
	    )
	    .subcommand(
		Command::new("search")
		    .short_flag('s')
		    .long_flag("search")
		    .about("Search for a project"),
	    )
	    .subcommand(
		Command::new("install-wrapper")
		    .short_flag('i')
		    .long_flag("install-wrapper")
		    .about("Install wrapper script to execute pcd"),
	    );

	ClapCli { command }
    }

    fn parse(&self) -> CliActions {
	let matches = self.command.clone().get_matches();

	match matches.subcommand() {
	    Some(("add", arg_matches)) => {
		let project = arg_matches.get_one::<String>("project_dir").cloned();

		if let Some(project) = &project {
		    match add(project) {
			Ok(_) => {}
			Err(e) => {
			    clap::Error::raw(clap::error::ErrorKind::InvalidValue, e).exit();
			}
		    }
		}

		CliActions::Add(project)
	    }
	    Some(("remove", arg_matches)) => {
		let project = arg_matches.get_one::<String>("project_name").cloned();

		CliActions::Remove(project)
	    }
	    Some(("search", _)) => CliActions::Search,
	    Some(("install-wrapper", _)) => {
		println!("Installing wrapper script...");
		install_wrapper();
		CliActions::InstallWrapper
	    }
	    None => {
		let project = matches.get_one::<String>("project_name").cloned();
		match project {
		    Some(project) => CliActions::Chdir(project),
		    None => CliActions::Search,
		}
	    }
	    _ => {
		unreachable!();
		// clap::Error::raw(clap::error::ErrorKind::InvalidValue, "Unknown command").exit();
	    }
	}
    }
}
