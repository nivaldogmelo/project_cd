use super::TomlConfig;

pub(super) fn read_config(config: String) -> Result<TomlConfig, String> {
    if config.is_empty() {
	Ok(TomlConfig { projects: vec![] })
    } else {
	match toml::from_str::<TomlConfig>(&config) {
	    Ok(projects) => Ok(projects),
	    Err(e) => Err(format!("Error parsing TOML: {}", e)),
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config() {
	let config = r#"[[projects]]
name = "test"
path = "/path/to/test"
[[projects]]
name = "test2"
path = "/path/to/test2"
"#;

	let result = read_config(config.to_string());
	// println!("{:?}", result);
	assert!(result.is_ok());

	let toml_config = result.unwrap();
	assert_eq!(toml_config.projects.len(), 2);

	assert_eq!(toml_config.projects[0].name, "test");
	assert_eq!(toml_config.projects[0].path, "/path/to/test");

	assert_eq!(toml_config.projects[1].name, "test2");
	assert_eq!(toml_config.projects[1].path, "/path/to/test2");
    }

    #[test]
    fn invalid_config() {
	let config = r#"[[projects]]
name = "test"
path = "/path/to/test"
[[projects]]
name = "test2"
"#;

	let result = read_config(config.to_string());
	assert!(result.is_err());
    }
}
