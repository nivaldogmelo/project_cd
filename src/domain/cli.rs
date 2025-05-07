pub trait Cli {
    fn new() -> Self;
    fn parse(&self) -> CliActions;
}

pub enum CliActions {
    Add(Option<String>),
    Chdir(String),
    InstallWrapper,
    Remove(Option<String>),
    Search,
    UnknownCommand,
}
