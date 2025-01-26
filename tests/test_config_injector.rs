use config_injector::process_files;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use tempfile::NamedTempFile;

#[test]
fn test_var_replacement() {
	// Simulate the environment variable
	env::set_var("SERVER_LICENSEKEY", "test_key");
	
	// Example file content
	let content = "sv_licenseKey {{SERVER_LICENSEKEY}}";
	
	// Create a temporary file and write content
	let mut temp_file = NamedTempFile::new().unwrap();
	temp_file.write_all(content.as_bytes()).unwrap();
	let path = temp_file.path().to_str().unwrap();
	
	// Prepare the environment variables map
	let env_vars = HashMap::from([("SERVER_LICENSEKEY".to_string(), "test_key".to_string())]);
	
	// Call the `process_files` function
	let files = vec![path];
	process_files(files, &env_vars).unwrap();
	
	// Verify the file content has been updated correctly
	let mut content = String::new();
	File::open(path).unwrap().read_to_string(&mut content).unwrap();
	assert_eq!(content, "sv_licenseKey test_key");
}

#[test]
fn test_missing_var() {
	// Create a temporary file with a placeholder for a missing variable
	let content = "sv_licenseKey {{SERVER_LICENSEKEY}}";
	let mut temp_file = NamedTempFile::new().unwrap();
	temp_file.write_all(content.as_bytes()).unwrap();
	let path = temp_file.path().to_str().unwrap();
	
	// Call the `process_files` function with an empty env_vars map
	let env_vars = HashMap::new(); // No SERVER_LICENSEKEY variable set
	let files = vec![path];
	process_files(files, &env_vars).unwrap();
	
	// Verify the file content has not been changed (placeholder remains)
	let mut content = String::new();
	File::open(path).unwrap().read_to_string(&mut content).unwrap();
	assert_eq!(content, "sv_licenseKey {{SERVER_LICENSEKEY}}");
}
