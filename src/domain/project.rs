use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Self {
	Project { name, path }
    }

    pub fn name(&self) -> &String {
	&self.name
    }

    pub fn path(&self) -> &PathBuf {
	&self.path
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "{} ({})", self.name, self.path.display())
    }
}
