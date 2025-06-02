use std::path::PathBuf;

pub trait Cli {
    fn new() -> Self;
    fn parse(&self) -> CliActions;
}

pub enum CliActions {
    Add(Result<PathBuf, String>),
    Chdir(String),
    InstallWrapper,
    Remove(Option<String>),
    Search,
    UnknownCommand,
}
