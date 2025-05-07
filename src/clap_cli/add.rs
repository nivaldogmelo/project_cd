use std::path::Path;

pub(super) fn add(project_dir: &str) -> Result<(), String> {
    let path = Path::new(project_dir);
    let path_exists = path.try_exists().map_err(|e| e.to_string())?;

    if path_exists {
	// Check if the path is a directory
	if path.is_dir() {
	    Ok(())
	} else {
	    Err(format!("Path is not a directory: {:?}", path))
	}
    } else {
	Err(format!("Path does not exist: {:?}", path))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_add() {
	let valid_dir = "/tmp";
	let invalid_dir = "/invalid/path";

	// Create a temporary file for testing
	let temp_file_dir = "/tmp/pcd_test_file";
	let _ = File::create(temp_file_dir);

	assert!(add(valid_dir).is_ok());
	assert!(add(invalid_dir).is_err());
	assert!(add(temp_file_dir).is_err());
    }
}
