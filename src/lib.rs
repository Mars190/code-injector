use std::fs::{File};
use std::io::{self, Read, Write};
use std::collections::HashMap;
use regex::Regex;
use log::{warn, info};
use std::path::Path;

/// Function to process files by replacing environment variable placeholders
/// with actual environment variable values.
pub fn process_files(files: Vec<&str>, env_vars: &HashMap<String, String>) -> io::Result<()> {
	let pattern = Regex::new(r"\{\{([A-Z_]+)\}\}").unwrap();
	
	for file in files {
		info!("Processing file: {}", file);
		
		let mut content = String::new();
		File::open(file)?.read_to_string(&mut content)?;
		
		// Replace placeholders with environment variable values
		let updated_content = pattern.replace_all(&content, |caps: &regex::Captures| {
			let var_name = &caps[1];
			match env_vars.get(&var_name.to_uppercase()) {
				Some(val) => val.to_string(),
				None => {
					warn!("Variable '{{{}}}' not found in environment variables", var_name);
					caps[0].to_string() // Return the original placeholder if not found
				}
			}
		});
		
		// Write the updated content back to the file
		let path = Path::new(file);
		let mut file = File::create(path)?;
		file.write_all(updated_content.as_bytes())?;
		
		info!("Updated file: {}", path.display());
	}
	
	Ok(())
}
