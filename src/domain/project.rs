use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
}

impl Project {
    pub fn new(name: String, path: String) -> Self {
	Project { name, path }
    }

    pub fn name(&self) -> &String {
	&self.name
    }

    pub fn path(&self) -> &String {
	&self.path
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "{} ({})", self.name, self.path)
    }
}
