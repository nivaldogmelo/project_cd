use std::path::{self, Path, PathBuf};

pub(super) fn add(project_dir: &str) -> Result<PathBuf, String> {
    let path = Path::new(project_dir);
    let path_exists = path.try_exists().map_err(|e| e.to_string())?;

    if path_exists {
	// Check if the path is a directory
	if !path.is_dir() {
	    return Err(format!("Path is not a directory: {:?}", path));
	}
    } else {
	return Err(format!("Path does not exist: {:?}", path));
    }

    if !path.is_absolute() {
	let absolute_path = path::absolute(path).map_err(|e| e.to_string())?;
	Ok(absolute_path)
    } else {
	Ok(path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, DirBuilder, File};

    use super::*;

    #[test]
    fn test_add() {
	let valid_dir = "/tmp";
	let invalid_dir = "/invalid/path";

	let res_valid = add(valid_dir);
	let res_invalid = add(invalid_dir);

	assert!(res_valid.is_ok());
	assert!(res_invalid.is_err());
    }

    #[test]
    fn test_file() {
	// Create a temporary file for testing
	let temp_file = "/tmp/pcd_test_file";
	let _ = File::create(temp_file);

	assert!(add(temp_file).is_err());
    }

    #[test]
    fn test_relative_path() {
	// Create a temporary dir for testing
	let temp_dir = "pcd_test_dir";
	let _ = DirBuilder::new().create(temp_dir);

	let res = add(temp_dir);

	assert!(res.is_ok());
	assert!(res.unwrap().is_absolute());

	fs::remove_dir(temp_dir).expect("Failed to remove temporary directory");
    }
}
